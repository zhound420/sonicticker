use std::collections::VecDeque;

use crate::models::{MarketMetrics, PriceTick};

pub struct IndicatorCalculator {
    symbol: String,
    period: usize,
    max_samples: usize,
    prices: VecDeque<f64>,
    volumes: VecDeque<f64>,
    returns: VecDeque<f64>,
    last_price: Option<f64>,
    open_price: Option<f64>,
}

impl IndicatorCalculator {
    pub fn new(symbol: impl Into<String>, period: usize, max_samples: usize) -> Self {
        Self {
            symbol: symbol.into(),
            period,
            max_samples,
            prices: VecDeque::with_capacity(max_samples),
            volumes: VecDeque::with_capacity(max_samples),
            returns: VecDeque::with_capacity(max_samples),
            last_price: None,
            open_price: None,
        }
    }

    pub fn on_tick(&mut self, tick: &PriceTick) -> MarketMetrics {
        Self::push_sample(&mut self.prices, self.max_samples, tick.price);
        Self::push_sample(&mut self.volumes, self.max_samples, tick.volume);

        if let Some(prev) = self.last_price {
            let r = ((tick.price / prev) - 1.0).clamp(-1.0, 1.0);
            Self::push_sample(&mut self.returns, self.max_samples, r);
        }

        self.last_price = Some(tick.price);
        self.open_price.get_or_insert(tick.price);

        MarketMetrics {
            symbol: self.symbol.clone(),
            price: tick.price,
            price_change_percent: self.price_change_pct(),
            volume: tick.volume,
            volume_ratio: self.volume_ratio(),
            rsi: self.rsi(),
            volatility: self.volatility(),
            tempo_bias: self.tempo_bias(),
            last_updated: tick.timestamp,
        }
    }

    fn push_sample(deque: &mut VecDeque<f64>, max_samples: usize, value: f64) {
        if deque.len() == max_samples {
            deque.pop_front();
        }
        deque.push_back(value);
    }

    fn price_change_pct(&self) -> f64 {
        match (self.open_price, self.last_price) {
            (Some(open), Some(last)) if open > 0.0 => ((last - open) / open) * 100.0,
            _ => 0.0,
        }
    }

    fn volume_ratio(&self) -> f64 {
        if self.volumes.is_empty() {
            return 1.0;
        }

        let current = *self.volumes.back().unwrap_or(&1.0);
        let avg = self.volumes.iter().copied().sum::<f64>() / self.volumes.len() as f64;
        if avg == 0.0 {
            1.0
        } else {
            (current / avg).clamp(0.1, 3.0)
        }
    }

    fn rsi(&self) -> f64 {
        if self.prices.len() < self.period + 1 {
            return 50.0;
        }

        let mut gains = 0.0;
        let mut losses = 0.0;

        let len = self.prices.len();
        let start = len.saturating_sub(self.period + 1);
        let window_prices: Vec<f64> = self.prices.iter().skip(start).copied().collect();
        for window in window_prices.windows(2) {
            let change = window[1] - window[0];
            if change > 0.0 {
                gains += change;
            } else {
                losses -= change;
            }
        }

        losses = losses.max(1e-9);
        let rs = gains / losses;
        100.0 - (100.0 / (1.0 + rs))
    }

    fn volatility(&self) -> f64 {
        if self.returns.is_empty() {
            return 0.0;
        }

        let mean = self.returns.iter().copied().sum::<f64>() / self.returns.len() as f64;
        let variance = self.returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>()
            / self.returns.len() as f64;
        (variance.sqrt() * 100.0).clamp(0.0, 10.0)
    }

    fn tempo_bias(&self) -> f64 {
        (self.volume_ratio() - 1.0).clamp(-0.5, 0.5)
    }
}

impl Default for IndicatorCalculator {
    fn default() -> Self {
        Self::new("UNKNOWN", 14, 256)
    }
}
