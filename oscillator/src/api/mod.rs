pub mod rest;
pub mod websocket;

use axum::{Router, http::Method, routing::get};
use tower_http::cors::{Any, CorsLayer};

use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .merge(rest::routes())
        .route("/ws/audio", get(websocket::upgrade))
        .layer(cors)
        .with_state(state)
}
