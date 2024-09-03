use core::panic;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;

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
use bittower::app::auth::ssr::{AuthSession, Session, AUTH_COOKIE};
use bittower::app::App;
use bittower::qbittorrent::client::QbtClient;
use bittower::qbittorrent::proto::sync::MainData;
use fileserv::file_and_error_handler;
use futures::stream::Stream;
use futures::{sink::SinkExt, stream::StreamExt};
use leptos::prelude::*;
use leptos_axum::{
    generate_route_list_with_exclusions_and_ssg_and_context, handle_server_fns_with_context,
    AxumRouteListing, LeptosRoutes,
};
use tower_http::compression::{
    predicate::{NotForContentType, SizeAbove},
    CompressionLayer, CompressionLevel, Predicate,
};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

pub mod fileserv;
mod handle_ws;
mod hashed_stylesheet;
mod hydration;
mod shell;

#[derive(Debug, axum::extract::FromRef, Clone)]
pub struct AppState {
    pub qbt: QbtClient,
    pub leptos_options: LeptosOptions,
    pub routes: Vec<AxumRouteListing>,
}

pub async fn serve() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let conf = get_configuration(None).unwrap();
    let mut leptos_options = conf.leptos_options;
    if cfg!(not(debug_assertions)) {
        leptos_options.hash_files = true;
    }
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
        let session = bittower::app::auth::ssr::get_session(sealed_token).ok();
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
            move || shell::shell(leptos_options.clone())
        },
    );

    handler(request).await.into_response()
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

    ws.on_upgrade(move |socket| handle_ws::handle_socket(socket, addr, app_state, session))
}
