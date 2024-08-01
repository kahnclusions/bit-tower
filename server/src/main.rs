use app::*;
use axum::{
    body::Body,
    extract::{FromRef, Path, Request, State},
    http::header::CONTENT_TYPE,
    response::IntoResponse,
    routing::{get, post},
    Router, ServiceExt,
};
use fileserv::file_and_error_handler;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, AxumRouteListing, LeptosRoutes};

pub mod fileserv;

#[derive(FromRef, Clone)]
pub struct AppState {
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
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        routes: routes.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    request: Request<Body>,
) -> axum::response::Response {
    let handler = leptos_axum::render_app_to_stream_with_context(move || {}, {
        let leptos_options = app_state.leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    handler(request).await.into_response()
}
