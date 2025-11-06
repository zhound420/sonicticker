import { useCallback, useEffect, useRef, useState } from 'react';
import type {
  AudioFrame,
  AudioMetadata,
  ConnectionStatus,
  MarketMetrics,
  MusicalParams,
} from '../types';

const WS_ENDPOINT =
  import.meta.env.VITE_WS_URL || `ws://${window.location.hostname}:8080/ws/audio`;

export const useAudioStream = (
  asset: string,
  onFrame: (frame: AudioFrame) => void,
) => {
  const wsRef = useRef<WebSocket | null>(null);
  const lastMetadata = useRef<AudioMetadata | null>(null);
  const [status, setStatus] = useState<ConnectionStatus>('idle');
  const [latestMetrics, setLatestMetrics] = useState<MarketMetrics | null>(null);
  const [latestParams, setLatestParams] = useState<MusicalParams | null>(null);

  const disconnect = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
  }, []);

  const connect = useCallback(() => {
    disconnect();
    if (!asset) {
      return;
    }

    setStatus('connecting');
    const ws = new WebSocket(`${WS_ENDPOINT}?asset=${asset}`);
    ws.binaryType = 'arraybuffer';

    ws.onopen = () => setStatus('streaming');
    ws.onerror = () => setStatus('error');
    ws.onclose = () => setStatus('idle');
    ws.onmessage = (event) => {
      if (typeof event.data === 'string') {
        const metadata = JSON.parse(event.data) as AudioMetadata;
        lastMetadata.current = metadata;
        setLatestMetrics(metadata.metrics);
        setLatestParams(metadata.params);
        return;
      }

      if (event.data instanceof ArrayBuffer && lastMetadata.current) {
        onFrame({
          metadata: lastMetadata.current,
          payload: event.data,
        });
        lastMetadata.current = null;
      }
    };

    wsRef.current = ws;
  }, [asset, disconnect, onFrame]);

  useEffect(() => {
    connect();
    return disconnect;
  }, [connect, disconnect]);

  return {
    status,
    latestMetrics,
    latestParams,
    reconnect: connect,
    disconnect,
  };
};
