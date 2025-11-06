import type { StyleDefinition } from '../types';

interface Props {
  styles: StyleDefinition[];
  selected: string;
  onSelect: (name: string) => void;
}

export const StyleSelector = ({ styles, selected, onSelect }: Props) => (
  <div className="panel">
    <h2>Styles</h2>
    <div className="style-grid">
      {styles.map((style) => (
        <button
          type="button"
          key={style.name}
          className={selected === style.name ? 'active' : ''}
          onClick={() => onSelect(style.name)}
        >
          <span className="symbol">{style.name}</span>
          <span className="description">{style.description}</span>
          <small>{style.instruments.join(' • ')}</small>
        </button>
      ))}
    </div>
  </div>
);
