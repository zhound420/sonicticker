import { useCallback, useEffect, useRef, useState } from 'react';
import './App.css';
import { AudioAnalyzer } from './audio/AudioAnalyzer';
import { AssetSelector } from './components/AssetSelector';
import { AudioVisualizer } from './components/AudioVisualizer';
import { MetricsDisplay } from './components/MetricsDisplay';
import { PriceChart } from './components/PriceChart';
import { StyleSelector } from './components/StyleSelector';
import { useAudioStream } from './hooks/useAudioStream';
import type {
  AssetDescriptor,
  AudioFrame,
  MarketMetrics,
  StyleDefinition,
} from './types';

const API_BASE =
  import.meta.env.VITE_API_URL || `http://${window.location.hostname}:8080`;

const STYLES: StyleDefinition[] = [
  {
    name: 'Electronic',
    instruments: ['Synth Lead', 'Sub Bass', '808 Drums'],
    description: 'High energy mapping for volatile assets.',
  },
  {
    name: 'Orchestral',
    instruments: ['Strings', 'Brass', 'Timpani'],
    description: 'Narrative textures for steady markets.',
  },
  {
    name: 'Ambient',
    instruments: ['Pads', 'Soft Percussion'],
    description: 'Ethereal soundscapes for reflective sessions.',
  },
  {
    name: 'Rock',
    instruments: ['Guitars', 'Live Bass', 'Kits'],
    description: 'Driven grooves for bullish momentum.',
  },
];

function App() {
  const analyzerRef = useRef<AudioAnalyzer | null>(null);
  const [assets, setAssets] = useState<AssetDescriptor[]>([]);
  const [selectedAsset, setSelectedAsset] = useState('');
  const [selectedStyle, setSelectedStyle] = useState(STYLES[0].name);
  const [history, setHistory] = useState<MarketMetrics[]>([]);
  const [volume, setVolume] = useState(0.8);

  useEffect(() => {
    analyzerRef.current = new AudioAnalyzer();
    analyzerRef.current.setVolume(volume);
    return () => {
      analyzerRef.current?.audioContext.close();
    };
  }, []);

  useEffect(() => {
    analyzerRef.current?.setVolume(volume);
  }, [volume]);

  useEffect(() => {
    fetch(`${API_BASE}/api/assets`)
      .then((response) => response.json())
      .then((data: AssetDescriptor[]) => {
        setAssets(data);
        if (!selectedAsset && data.length > 0) {
          setSelectedAsset(data[0].symbol);
        }
      })
      .catch((err) => {
        console.error('Failed to load assets', err);
      });
  }, []);

  const handleFrame = useCallback((frame: AudioFrame) => {
    void analyzerRef.current?.playFrame(frame);
  }, []);

  const { status, latestMetrics, latestParams, reconnect, disconnect } =
    useAudioStream(selectedAsset, handleFrame);

  useEffect(() => {
    if (!latestMetrics) {
      return;
    }
    setHistory((prev) => {
      const next = [...prev, latestMetrics];
      return next.slice(-200);
    });
  }, [latestMetrics]);

  return (
    <div className="app">
      <header>
        <div>
          <h1>Oscillator</h1>
          <p>Real-time market sonification and visual synthesis</p>
        </div>
        <div className="controls">
          <button type="button" onClick={reconnect}>
            Start
          </button>
          <button type="button" onClick={disconnect}>
            Stop
          </button>
          <label>
            Volume
            <input
              type="range"
              min={0}
              max={1}
              step={0.01}
              value={volume}
              onChange={(event) => setVolume(Number(event.target.value))}
            />
          </label>
        </div>
      </header>

      <main>
        <section className="sidebar">
          <AssetSelector
            assets={assets}
            selected={selectedAsset}
            onSelect={setSelectedAsset}
          />
          <StyleSelector
            styles={STYLES}
            selected={selectedStyle}
            onSelect={setSelectedStyle}
          />
          <MetricsDisplay
            metrics={latestMetrics}
            params={latestParams}
            status={status}
          />
        </section>
        <section className="stage">
          <AudioVisualizer
            analyzer={analyzerRef.current}
            metrics={latestMetrics}
          />
          <PriceChart history={history} />
        </section>
      </main>
    </div>
  );
}

export default App;
