import type { ConnectionStatus, MarketMetrics, MusicalParams } from '../types';

interface Props {
  metrics: MarketMetrics | null;
  params: MusicalParams | null;
  status: ConnectionStatus;
}

const format = (value?: number, digits = 2) =>
  typeof value === 'number' ? value.toFixed(digits) : '—';

export const MetricsDisplay = ({ metrics, params, status }: Props) => (
  <div className="panel">
    <h2>Live Metrics</h2>
    <div className="metrics-grid">
      <Metric label="Status" value={status} />
      <Metric label="Price" value={metrics ? `$${format(metrics.price, 2)}` : '—'} />
      <Metric
        label="Change %"
        value={metrics ? `${format(metrics.price_change_percent)}%` : '—'}
      />
      <Metric
        label="Volume Ratio"
        value={metrics ? format(metrics.volume_ratio, 2) : '—'}
      />
      <Metric label="RSI" value={metrics ? format(metrics.rsi, 1) : '—'} />
      <Metric
        label="Volatility"
        value={metrics ? format(metrics.volatility, 2) : '—'}
      />
      <Metric label="Tempo" value={params ? `${format(params.tempo, 1)} BPM` : '—'} />
      <Metric
        label="Style"
        value={params ? params.style : '—'}
      />
    </div>
  </div>
);

const Metric = ({ label, value }: { label: string; value: string }) => (
  <div className="metric">
    <span>{label}</span>
    <strong>{value}</strong>
  </div>
);
