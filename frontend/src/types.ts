export type AssetCategory = 'crypto' | 'stock';

export interface AssetDescriptor {
  symbol: string;
  display_name: string;
  category: AssetCategory;
  description: string;
}

export interface MarketMetrics {
  symbol: string;
  price: number;
  price_change_percent: number;
  volume: number;
  volume_ratio: number;
  rsi: number;
  volatility: number;
  tempo_bias: number;
  last_updated: string;
}

export interface MusicalParams {
  tempo: number;
  melody_notes: number[];
  bass_note: number;
  harmony: 'Major' | 'Minor' | 'Diminished' | 'Suspended';
  reverb_mix: number;
  distortion: number;
  volume_intensity: number;
  style: string;
}

export interface AudioMetadata {
  asset: string;
  sample_rate: number;
  frames: number;
  channels: number;
  timestamp: string;
  metrics: MarketMetrics;
  params: MusicalParams;
  payload_bytes: number;
}

export interface AudioFrame {
  metadata: AudioMetadata;
  payload: ArrayBuffer;
}

export type ConnectionStatus = 'idle' | 'connecting' | 'streaming' | 'error';

export interface StyleDefinition {
  name: string;
  instruments: string[];
  description: string;
}
