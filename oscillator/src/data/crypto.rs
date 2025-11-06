use std::time::Duration;

use chrono::{TimeZone, Utc};
use futures::StreamExt;
use serde::Deserialize;
use tokio::{
    sync::mpsc::Sender,
    task::JoinHandle,
    time::{sleep, timeout},
};
use tokio_tungstenite::connect_async;
use tracing::{error, info, warn};

use crate::models::PriceTick;

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Debug, Clone)]
pub struct BinanceClient {
    endpoint: String,
}

impl BinanceClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
        }
    }

    pub fn spawn_trade_stream(
        &self,
        symbol: impl Into<String>,
        tx: Sender<PriceTick>,
    ) -> JoinHandle<()> {
        let symbol = symbol.into();
        let endpoint = self.endpoint.clone();
        tokio::spawn(async move {
            loop {
                if let Err(err) = Self::run_stream(&endpoint, &symbol, &tx).await {
                    error!(%symbol, %err, "Binance stream failed");
                }
                warn!(%symbol, "Reconnecting Binance stream in 3s");
                sleep(Duration::from_secs(3)).await;
            }
        })
    }

    async fn run_stream(
        endpoint: &str,
        symbol: &str,
        tx: &Sender<PriceTick>,
    ) -> anyhow::Result<()> {
        let url = format!(
            "{}/ws/{}@trade",
            endpoint.trim_end_matches('/'),
            symbol.to_lowercase()
        );
        info!(%symbol, %url, "Connecting to Binance");

        let (ws_stream, _) = timeout(CONNECTION_TIMEOUT, connect_async(&url)).await??;
        let (_, mut stream) = ws_stream.split();

        while let Some(message) = timeout(HEARTBEAT_TIMEOUT, stream.next()).await? {
            let message = message?;
            if !message.is_text() {
                continue;
            }
            let trade: BinanceTrade = serde_json::from_str(message.to_text()?)?;

            let price = trade.p.parse::<f64>().unwrap_or_default();
            let volume = trade.q.parse::<f64>().unwrap_or(0.0);
            let ts = Utc
                .timestamp_millis_opt(trade.event_time)
                .single()
                .unwrap_or_else(|| Utc::now());

            let tick = PriceTick {
                symbol: symbol.to_string(),
                price,
                volume,
                timestamp: ts,
            };

            if tx.send(tick).await.is_err() {
                break;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct BinanceTrade {
    #[serde(rename = "p")]
    p: String,
    #[serde(rename = "q")]
    q: String,
    #[serde(rename = "T")]
    event_time: i64,
}
