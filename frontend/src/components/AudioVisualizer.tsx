import { useEffect, useRef } from 'react';
import type { AudioAnalyzer } from '../audio/AudioAnalyzer';
import type { MarketMetrics } from '../types';
import { ParticleSystem } from '../visualizations/ParticleSystem';
import { renderFrequencyBars } from '../visualizations/FrequencyBars';
import { renderWaveform } from '../visualizations/WaveformRenderer';

interface Props {
  analyzer: AudioAnalyzer | null;
  metrics: MarketMetrics | null;
}

const particles = new ParticleSystem();

export const AudioVisualizer = ({ analyzer, metrics }: Props) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !analyzer) {
      return;
    }
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      return;
    }

    let frame = 0;
    let last = performance.now();
    const loop = (time: number) => {
      const delta = (time - last) / 16.6;
      last = time;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle =
        metrics && metrics.price_change_percent >= 0 ? '#041c18' : '#1b0b0f';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const waveform = analyzer.getWaveform();
      const freqs = analyzer.getFrequencies();
      const bullish = (metrics?.price_change_percent ?? 0) >= 0;
      const activity = metrics ? metrics.volume_ratio : 1;

      renderWaveform(ctx, waveform, bullish ? '#34d399' : '#fb7185');
      renderFrequencyBars(ctx, freqs);
      particles.update(ctx, activity, bullish, delta);

      frame = requestAnimationFrame(loop);
    };
    frame = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(frame);
  }, [analyzer, metrics]);

  return (
    <div className="panel visualizer">
      <h2>Audio Visualizer</h2>
      <canvas ref={canvasRef} width={800} height={320} />
    </div>
  );
};
