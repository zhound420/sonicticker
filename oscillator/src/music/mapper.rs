use tunes::prelude::*;

use crate::models::{HarmonyQuality, MarketMetrics, MusicalParams};

use super::styles::CompositionStyle;

const MAJOR_PENT: [f32; 5] = [C4, D4, E4, G4, A4];
const MINOR_PENT: [f32; 5] = [A3, C4, D4, E4, G4];
const WHOLE_TONE: [f32; 6] = [C4, D4, E4, FS4, GS4, AS4];

pub struct MarketMapper {
    base_tempo: f64,
}

impl MarketMapper {
    pub fn new(base_tempo: f64) -> Self {
        Self { base_tempo }
    }

    pub fn map(&self, metrics: &MarketMetrics, style: CompositionStyle) -> MusicalParams {
        let (scale, ascending) = self.scale_for(metrics.price_change_percent);
        let idx = self.scale_index(metrics.price_change_percent, scale.len());
        let melody_note = if ascending {
            scale[idx]
        } else {
            scale[scale.len() - idx - 1]
        };

        let bass_note = self.bass_from_price(metrics.price);

        let tempo =
            self.base_tempo + (metrics.volume_ratio - 1.0) * 30.0 + metrics.tempo_bias * 40.0;
        let tempo = tempo.clamp(80.0, 160.0);

        let harmony = if metrics.rsi < 30.0 {
            HarmonyQuality::Minor
        } else if metrics.rsi > 70.0 {
            HarmonyQuality::Diminished
        } else {
            HarmonyQuality::Major
        };

        let reverb_mix = (metrics.volatility / 5.0).clamp(0.05, 0.7) as f32;
        let distortion = (metrics.volatility / 3.0).clamp(0.0, 0.8) as f32;

        MusicalParams {
            tempo,
            melody_notes: vec![melody_note],
            bass_note,
            harmony,
            reverb_mix,
            distortion,
            volume_intensity: metrics.volume_ratio,
            style: style.as_str().to_string(),
        }
    }

    fn scale_for(&self, price_change_percent: f64) -> (&'static [f32], bool) {
        if price_change_percent > 5.0 || price_change_percent < -5.0 {
            return (&WHOLE_TONE, price_change_percent >= 0.0);
        }

        if price_change_percent >= 0.0 {
            (&MAJOR_PENT, true)
        } else {
            (&MINOR_PENT, false)
        }
    }

    fn scale_index(&self, pct: f64, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        let norm = (pct.abs() / 10.0).clamp(0.0, 1.0);
        (norm * (len - 1) as f64).round() as usize
    }

    fn bass_from_price(&self, price: f64) -> f32 {
        const LOW: f32 = C1;
        const HIGH: f32 = C3;

        if price <= 0.0 {
            return LOW;
        }

        let normalized = (price.log10() / 5.0).clamp(0.0, 1.0) as f32;
        LOW + (HIGH - LOW) * normalized
    }
}
