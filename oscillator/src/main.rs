use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use oscillator::{
    api,
    state::{AppConfig, AppState, MarketEngine},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "oscillator=info,tunes=warn,axum=info".into());
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();

    let config = AppConfig::from_env();
    let state = AppState::new(config.clone());
    let engine = MarketEngine::new(state.clone());
    engine.spawn();

    let router: Router = api::build_router(state.clone());

    let listener = TcpListener::bind(config.addr()).await?;
    info!(addr = %config.addr(), "Oscillator listening");

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
