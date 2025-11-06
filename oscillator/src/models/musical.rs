use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::MarketMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarmonyQuality {
    Major,
    Minor,
    Diminished,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicalParams {
    pub tempo: f64,
    pub melody_notes: Vec<f32>,
    pub bass_note: f32,
    pub harmony: HarmonyQuality,
    pub reverb_mix: f32,
    pub distortion: f32,
    pub volume_intensity: f64,
    pub style: String,
}

impl Default for MusicalParams {
    fn default() -> Self {
        Self {
            tempo: 100.0,
            melody_notes: vec![],
            bass_note: 65.41,
            harmony: HarmonyQuality::Major,
            reverb_mix: 0.2,
            distortion: 0.0,
            volume_intensity: 1.0,
            style: "Electronic".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChunk {
    pub samples: Vec<u8>,
    pub frames: usize,
    pub channels: u8,
    pub sample_rate: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioPacket {
    pub asset: String,
    pub metrics: MarketMetrics,
    pub params: MusicalParams,
    pub chunk: AudioChunk,
}
