use core::panic;

use app::*;
use auth::ssr::{AuthSession, Session, AUTH_COOKIE};
use axum::response::sse::{Event, Sse};
use axum::{
    body::Body,
    extract::{FromRef, Path, Request, State},
    http::header,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use fileserv::file_and_error_handler;
use futures::stream::Stream;
use leptos::prelude::*;
use leptos_axum::{
    generate_route_list_with_exclusions_and_ssg_and_context, handle_server_fns_with_context,
    AxumRouteListing, LeptosRoutes,
};
use qbittorrent_rs::QbtClient;

pub mod fileserv;

#[derive(FromRef, Clone)]
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

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .route("/sse", get(handle_sse))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(middleware::from_fn(session_middleware))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
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
