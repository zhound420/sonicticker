import type { AssetDescriptor, AssetCategory } from '../types';

interface Props {
  assets: AssetDescriptor[];
  selected: string;
  onSelect: (symbol: string) => void;
}

const categoryLabels: Record<AssetCategory, string> = {
  crypto: 'Crypto',
  stock: 'Stocks',
};

export const AssetSelector = ({ assets, selected, onSelect }: Props) => {
  const grouped = assets.reduce<Record<AssetCategory, AssetDescriptor[]>>(
    (acc, asset) => {
      acc[asset.category] = acc[asset.category] || [];
      acc[asset.category].push(asset);
      return acc;
    },
    { crypto: [], stock: [] },
  );

  return (
    <div className="panel">
      <h2>Assets</h2>
      {Object.entries(grouped).map(([category, list]) => (
        <div key={category} className="asset-group">
          <h3>{categoryLabels[category as AssetCategory]}</h3>
          <div className="asset-grid">
            {list.map((asset) => (
              <button
                type="button"
                key={asset.symbol}
                className={selected === asset.symbol ? 'active' : ''}
                onClick={() => onSelect(asset.symbol)}
              >
                <span className="symbol">{asset.display_name}</span>
                <span className="description">{asset.description}</span>
              </button>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
};
