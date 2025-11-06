import { useEffect, useRef } from 'react';
import type { MarketMetrics } from '../types';

interface Props {
  history: MarketMetrics[];
}

export const PriceChart = ({ history }: Props) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || history.length < 2) {
      return;
    }
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      return;
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const prices = history.map((point) => point.price);
    const min = Math.min(...prices);
    const max = Math.max(...prices);
    const range = max - min || 1;
    ctx.strokeStyle = '#34d399';
    ctx.lineWidth = 2;
    ctx.beginPath();
    history.forEach((point, index) => {
      const x = (index / (history.length - 1)) * canvas.width;
      const normalized = (point.price - min) / range;
      const y = canvas.height - normalized * canvas.height;
      if (index === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });
    ctx.stroke();
  }, [history]);

  return (
    <div className="panel">
      <h2>Price Trace</h2>
      <canvas ref={canvasRef} width={600} height={160} />
    </div>
  );
};
