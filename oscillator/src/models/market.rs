use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AssetCategory {
    Crypto,
    Stock,
}

impl AssetCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Crypto => "crypto",
            Self::Stock => "stock",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDescriptor {
    pub symbol: String,
    pub display_name: String,
    pub category: AssetCategory,
    pub description: String,
    pub tick_size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSelection {
    pub symbol: String,
    pub category: AssetCategory,
    pub style_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTick {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketMetrics {
    pub symbol: String,
    pub price: f64,
    pub price_change_percent: f64,
    pub volume: f64,
    pub volume_ratio: f64,
    pub rsi: f64,
    pub volatility: f64,
    pub tempo_bias: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetKind {
    BinanceTrade { symbol: String },
    YahooEquity { symbol: String },
}
