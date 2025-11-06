use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use parking_lot::RwLock;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::{
    data::{
        crypto::BinanceClient,
        indicators::IndicatorCalculator,
        stocks::YahooFinanceClient,
        streams::{self, TickReceiver},
    },
    models::{AssetCategory, AssetDescriptor, AudioPacket, MarketMetrics},
    music::{MarketComposer, MarketMapper, StylePalette},
};

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub binance_ws: String,
    pub yahoo_base: String,
    pub sample_rate: u32,
    pub chunk_bars: usize,
    pub base_tempo: f64,
    pub assets: Vec<AssetDescriptor>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("OSC_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = std::env::var("OSC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);
        let binance_ws = std::env::var("OSC_BINANCE_WS")
            .unwrap_or_else(|_| "wss://stream.binance.com:9443".into());
        let yahoo_base = std::env::var("OSC_YAHOO_BASE")
            .unwrap_or_else(|_| "https://query1.finance.yahoo.com".into());
        let sample_rate = std::env::var("OSC_SAMPLE_RATE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(44_100);
        let chunk_bars = std::env::var("OSC_CHUNK_BARS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2);
        let base_tempo = std::env::var("OSC_BASE_TEMPO")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(104.0);

        Self {
            host,
            port,
            binance_ws,
            yahoo_base,
            sample_rate,
            chunk_bars,
            base_tempo,
            assets: default_assets(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(
            self.host
                .parse()
                .unwrap_or_else(|_| "0.0.0.0".parse().expect("host parse")),
            self.port,
        )
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    config: AppConfig,
    assets: Vec<AssetDescriptor>,
    metrics: RwLock<HashMap<String, MarketMetrics>>,
    broadcasters: RwLock<HashMap<String, broadcast::Sender<AudioPacket>>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let mut broadcasters = HashMap::new();
        for asset in &config.assets {
            let (tx, _) = broadcast::channel(32);
            broadcasters.insert(asset.symbol.clone(), tx);
        }

        Self {
            inner: Arc::new(AppStateInner {
                assets: config.assets.clone(),
                config,
                metrics: RwLock::new(HashMap::new()),
                broadcasters: RwLock::new(broadcasters),
            }),
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.inner.config
    }

    pub fn assets(&self) -> &[AssetDescriptor] {
        &self.inner.assets
    }

    pub fn latest_metrics(&self, symbol: &str) -> Option<MarketMetrics> {
        self.inner.metrics.read().get(symbol).cloned()
    }

    pub fn update_metrics(&self, metrics: MarketMetrics) {
        self.inner
            .metrics
            .write()
            .insert(metrics.symbol.clone(), metrics);
    }

    pub fn subscribe(&self, symbol: &str) -> broadcast::Receiver<AudioPacket> {
        let mut broadcasters = self.inner.broadcasters.write();
        let entry = broadcasters.entry(symbol.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(32);
            tx
        });
        entry.subscribe()
    }

    pub fn publish(&self, packet: AudioPacket) {
        self.update_metrics(packet.metrics.clone());
        if let Some(tx) = self.inner.broadcasters.read().get(&packet.asset) {
            let _ = tx.send(packet);
        }
    }
}

pub struct MarketEngine {
    state: AppState,
    binance: BinanceClient,
    yahoo: YahooFinanceClient,
    palette: StylePalette,
}

impl MarketEngine {
    pub fn new(state: AppState) -> Self {
        let config = state.config().clone();
        Self {
            binance: BinanceClient::new(config.binance_ws.clone()),
            yahoo: YahooFinanceClient::new(config.yahoo_base.clone()),
            palette: StylePalette::default(),
            state,
        }
    }

    pub fn spawn(&self) {
        for asset in self.state.assets() {
            self.spawn_asset(asset.clone());
        }
    }

    fn spawn_asset(&self, asset: AssetDescriptor) {
        let (tx, rx) = streams::channel(512);
        match asset.category {
            AssetCategory::Crypto => {
                self.binance.spawn_trade_stream(asset.symbol.clone(), tx);
            }
            AssetCategory::Stock => {
                self.yahoo.spawn_price_poller(asset.symbol.clone(), tx, 15);
            }
        }

        let composer = MarketComposer::new(
            self.state.config().sample_rate,
            self.state.config().chunk_bars,
        );
        let mapper = MarketMapper::new(self.state.config().base_tempo);
        let palette = self.palette.clone();
        let state = self.state.clone();

        tokio::spawn(async move {
            run_pipeline(asset, rx, mapper, composer, palette, state).await;
        });
    }
}

async fn run_pipeline(
    asset: AssetDescriptor,
    mut rx: TickReceiver,
    mapper: MarketMapper,
    composer: MarketComposer,
    palette: StylePalette,
    state: AppState,
) {
    let mut indicators = IndicatorCalculator::new(&asset.symbol, 14, 512);
    info!(symbol = %asset.symbol, "Pipeline started");

    while let Some(tick) = rx.recv().await {
        let metrics = indicators.on_tick(&tick);
        let style = palette.style_for_category(asset.category.clone(), metrics.volatility >= 2.5);
        let params = mapper.map(&metrics, style);

        match composer.render_chunk(&params, style) {
            Ok(chunk) => {
                state.publish(AudioPacket {
                    asset: asset.symbol.clone(),
                    metrics: metrics.clone(),
                    params: params.clone(),
                    chunk,
                });
            }
            Err(err) => {
                error!(symbol = %asset.symbol, %err, "composer failure");
            }
        }
    }

    warn!(symbol = %asset.symbol, "Pipeline terminated");
}

fn default_assets() -> Vec<AssetDescriptor> {
    vec![
        AssetDescriptor {
            symbol: "btcusdt".to_string(),
            display_name: "BTC/USDT".to_string(),
            category: AssetCategory::Crypto,
            description: "Bitcoin vs Tether spot market (Binance)".to_string(),
            tick_size: 0.01,
        },
        AssetDescriptor {
            symbol: "ethusdt".to_string(),
            display_name: "ETH/USDT".to_string(),
            category: AssetCategory::Crypto,
            description: "Ethereum vs Tether".to_string(),
            tick_size: 0.01,
        },
        AssetDescriptor {
            symbol: "solusdt".to_string(),
            display_name: "SOL/USDT".to_string(),
            category: AssetCategory::Crypto,
            description: "Solana vs Tether".to_string(),
            tick_size: 0.01,
        },
        AssetDescriptor {
            symbol: "AAPL".to_string(),
            display_name: "Apple Inc.".to_string(),
            category: AssetCategory::Stock,
            description: "Apple equity (NASDAQ)".to_string(),
            tick_size: 0.01,
        },
        AssetDescriptor {
            symbol: "TSLA".to_string(),
            display_name: "Tesla Inc.".to_string(),
            category: AssetCategory::Stock,
            description: "Tesla equity (NASDAQ)".to_string(),
            tick_size: 0.01,
        },
        AssetDescriptor {
            symbol: "SPY".to_string(),
            display_name: "S&P 500 ETF".to_string(),
            category: AssetCategory::Stock,
            description: "SPDR S&P 500 ETF".to_string(),
            tick_size: 0.01,
        },
    ]
}
