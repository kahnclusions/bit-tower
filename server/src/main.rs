use core::panic;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;

use app::*;
use auth::ssr::{AuthSession, Session, AUTH_COOKIE};
use axum::extract::ws::CloseFrame;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::response::sse::{Event, Sse};
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        FromRef, Path, Request, State, WebSocketUpgrade,
    },
    http::header,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use fileserv::file_and_error_handler;
use futures::stream::Stream;
use futures::{sink::SinkExt, stream::StreamExt};
use leptos::prelude::*;
use leptos_axum::{
    generate_route_list_with_exclusions_and_ssg_and_context, handle_server_fns_with_context,
    AxumRouteListing, LeptosRoutes,
};
use qbittorrent_rs::QbtClient;
use qbittorrent_rs_proto::sync::MainData;
use tower_http::compression::{
    predicate::{NotForContentType, SizeAbove},
    CompressionLayer, CompressionLevel, Predicate,
};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

pub mod fileserv;
// mod handle_ws;

#[derive(Debug, FromRef, Clone)]
pub struct AppState {
    pub qbt: QbtClient,
    pub leptos_options: LeptosOptions,
    pub routes: Vec<AxumRouteListing>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let qbt = QbtClient::new("http://localhost:9090/api/v2");
    let qbt_routes = qbt.clone();

    let (routes, _static_data_map) =
        generate_route_list_with_exclusions_and_ssg_and_context(App, None, move || {
            provide_context::<QbtClient>(qbt_routes.clone());
        });

    let app_state = AppState {
        qbt,
        leptos_options: leptos_options.clone(),
        routes: routes.clone(),
    };

    let predicate = SizeAbove::new(1500) // files smaller than 1501 bytes are not compressed, since the MTU (Maximum Transmission Unit) of a TCP packet is 1500 bytes
        .and(NotForContentType::GRPC)
        .and(NotForContentType::IMAGES)
        // prevent compressing assets that are already statically compressed
        .and(NotForContentType::const_new("application/javascript"))
        .and(NotForContentType::const_new("application/wasm"))
        .and(NotForContentType::const_new("text/css"));

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .route("/ws", get(ws_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        // .layer(
        //     CompressionLayer::new()
        //         .quality(CompressionLevel::Fastest)
        //         .compress_when(predicate),
        // )
        .fallback(file_and_error_handler)
        .layer(middleware::from_fn(session_middleware))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn session_middleware(mut request: Request, next: Next) -> Response {
    let res = request
        .headers()
        .get_all(header::COOKIE)
        .into_iter()
        .find_map(|x| {
            let cs = cookie::Cookie::split_parse(x.to_str().unwrap());
            let token = cs
                .into_iter()
                .find(|c| match c {
                    Ok(c) => c.name() == AUTH_COOKIE,
                    _ => false,
                })
                .map(|c| c.unwrap().value().to_string());
            token
        });
    request.extensions_mut().insert(AuthSession::new(None));
    if let Some(sealed_token) = res {
        let session = app::auth::ssr::get_session(sealed_token).ok();
        if let Some(session) = session {
            request
                .extensions_mut()
                .insert(AuthSession::new(Some(session)));
        }
    }
    next.run(request).await
}

/// Creates an axum handler to inject context into server functions.
async fn server_fn_handler(
    State(app_state): State<AppState>,
    Extension(auth_session): Extension<AuthSession>,
    path: Path<String>,
    request: Request<Body>,
) -> impl IntoResponse {
    tracing::info!("Handling server function request: {:?}", path);
    handle_server_fns_with_context(
        move || {
            provide_context::<QbtClient>(app_state.qbt.clone());
            if let Some(session) = &auth_session.session {
                provide_context::<Session>(session.clone());
            }
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    Extension(auth_session): Extension<AuthSession>,
    request: Request<Body>,
) -> axum::response::Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context::<QbtClient>(app_state.qbt.clone());
            if let Some(session) = &auth_session.session {
                provide_context::<Session>(session.clone());
            }
        },
        {
            let leptos_options = app_state.leptos_options.clone();
            move || shell(leptos_options.clone())
        },
    );

    handler(request).await.into_response()
}

async fn handle_sse(
    State(app_state): State<AppState>,
    Extension(auth_session): Extension<AuthSession>,
) -> Sse<impl Stream<Item = Result<Event, axum::BoxError>>> {
    let Some(session) = auth_session.session else {
        panic!("Unauthenticated");
    };

    qbittorrent_rs_sse::handle_sse(app_state.qbt.clone(), session.sid).await
}

#[tracing::instrument(skip(ws))]
async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    State(app_state): State<AppState>,
    Extension(auth_session): Extension<AuthSession>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    tracing::info!("Got a WS connection");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` at {addr} connected.");

    let Some(session) = auth_session.session else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    ws.on_upgrade(move |socket| handle_socket(socket, addr, app_state, session))
}

/// Actual websocket statemachine (one will be spawned per connection)
#[tracing::instrument(skip(socket))]
async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    app_state: AppState,
    session: Session,
) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        tracing::info!("Pinged {who}...");
    } else {
        tracing::info!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            tracing::info!("client {who} abruptly disconnected");
            return;
        }
    }

    // Since each client gets individual statemachine, we can pause handling
    // when necessary to wait for some external event (in this case illustrated by sleeping).
    // Waiting for this client to finish getting its greetings does not prevent other clients from
    // connecting to server and receiving their greetings.
    for i in 1..5 {
        if socket
            .send(Message::Text(format!("Hi {i} times!")))
            .await
            .is_err()
        {
            tracing::info!("client {who} abruptly disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let mut rid = 0_u64;
        let sid = session.sid.as_str();
        loop {
            tracing::info!("Syncing QBT maindata -->");
            // handle_ws(app_state.qbt.clone(), session.sid).await
            let res = app_state.qbt.sync_maindata(sid, rid).await;

            let Ok(maindata) = res else {
                tracing::error!("Oops, the thing went boom.");
                break;
                // return Err(axum::BoxError::from(anyhow!("Oops")));
            };
            rid = match &maindata {
                MainData::Full(md) => md.rid,
                MainData::Partial(md) => md.rid,
            };
            let res = sender
                .send(Message::Binary(rmp_serde::to_vec(&maindata).unwrap()))
                .await;
            if let Err(err) = res {
                tracing::error!(error = %err);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
        rid
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => tracing::info!("{a} messages sent to {who}"),
                Err(a) => tracing::info!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => tracing::info!("Received {b} messages"),
                Err(b) => tracing::info!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    tracing::info!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
#[tracing::instrument]
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            tracing::info!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            tracing::info!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::info!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            tracing::info!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
