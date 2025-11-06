use std::{sync::Arc, time::Duration};

use chrono::{TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;
use tokio::{
    sync::mpsc::Sender,
    task::JoinHandle,
    time::{interval, sleep},
};
use tracing::{info, warn};

use crate::models::PriceTick;

#[derive(Clone)]
pub struct YahooFinanceClient {
    base_url: Arc<String>,
    client: Client,
}

impl YahooFinanceClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Arc::new(base_url.into()),
            client: Client::builder()
                .user_agent("oscillator/0.1")
                .build()
                .expect("reqwest client"),
        }
    }

    pub fn spawn_price_poller(
        &self,
        symbol: impl Into<String>,
        tx: Sender<PriceTick>,
        interval_secs: u64,
    ) -> JoinHandle<()> {
        let symbol = symbol.into();
        let client = self.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(interval_secs));
            loop {
                ticker.tick().await;
                match client.fetch_latest(&symbol).await {
                    Ok(tick) => {
                        if tx.send(tick).await.is_err() {
                            break;
                        }
                    }
                    Err(err) => {
                        warn!(%symbol, %err, "Yahoo poll failed - backing off");
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        })
    }

    async fn fetch_latest(&self, symbol: &str) -> anyhow::Result<PriceTick> {
        let url = format!(
            "{}/v8/finance/chart/{}?interval=1m&range=1d",
            self.base_url, symbol
        );
        info!(%symbol, "Fetching Yahoo chart");
        let resp: ChartResponse = self.client.get(url).send().await?.json().await?;
        let result = resp
            .chart
            .result
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("empty chart result"))?;
        let quote = result
            .indicators
            .quote
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("missing quote block"))?;

        let mut latest = None;

        for (idx, ts) in result.timestamp.iter().enumerate().rev() {
            if let (Some(price), Some(volume)) = (quote.close.get(idx), quote.volume.get(idx)) {
                if let (Some(price), Some(volume)) = (price, volume) {
                    latest = Some((*ts, *price, *volume));
                    break;
                }
            }
        }

        let (ts, price, volume) = latest.ok_or_else(|| anyhow::anyhow!("no data points"))?;
        let timestamp = Utc
            .timestamp_opt(ts, 0)
            .single()
            .unwrap_or_else(|| Utc::now());

        Ok(PriceTick {
            symbol: symbol.to_string(),
            price,
            volume,
            timestamp,
        })
    }
}

#[derive(Debug, Deserialize)]
struct ChartResponse {
    chart: ChartResult,
}

#[derive(Debug, Deserialize)]
struct ChartResult {
    result: Vec<ChartEntry>,
}

#[derive(Debug, Deserialize)]
struct ChartEntry {
    timestamp: Vec<i64>,
    indicators: IndicatorBlock,
}

#[derive(Debug, Deserialize)]
struct IndicatorBlock {
    quote: Vec<QuoteBlock>,
}

#[derive(Debug, Deserialize)]
struct QuoteBlock {
    close: Vec<Option<f64>>,
    volume: Vec<Option<f64>>,
}
