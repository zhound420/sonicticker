use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Serialize;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/api/assets", get(list_assets))
        .route("/api/metrics/:symbol", get(latest_metrics))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        message: "Oscillator backend alive",
    })
}

async fn list_assets(State(state): State<AppState>) -> Json<Vec<AssetSummary>> {
    let assets = state
        .assets()
        .iter()
        .map(|asset| AssetSummary {
            symbol: asset.symbol.clone(),
            display_name: asset.display_name.clone(),
            category: asset.category.as_str().to_string(),
            description: asset.description.clone(),
        })
        .collect();
    Json(assets)
}

async fn latest_metrics(
    Path(symbol): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<crate::models::MarketMetrics>, StatusCode> {
    state
        .latest_metrics(&symbol)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    message: &'static str,
}

#[derive(Serialize)]
struct AssetSummary {
    symbol: String,
    display_name: String,
    category: String,
    description: String,
}
