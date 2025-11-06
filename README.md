## Oscillator – Real-Time Market Sonification

Oscillator converts live market data into evolving music and browser-based visualizations. The Rust backend ingests Binance and Yahoo Finance feeds, maps market indicators to musical parameters using the [`tunes`](https://github.com/sqrew/tunes) library, and streams audio buffers plus telemetry over WebSockets. A React/Vite frontend plays the audio with the Web Audio API, renders FFT-driven visuals, and exposes controls for asset/style selection.

### Repository Layout

```
oscillator/        # Rust backend
├─ src/api         # REST + WebSocket endpoints
├─ src/data        # Market ingestion + indicators
├─ src/music       # Mapper + tunes-based composer
├─ src/models      # Shared data structures
└─ src/state       # AppState + pipeline orchestration

frontend/          # React + TypeScript + Vite client
├─ src/components  # UI widgets & visualizers
├─ src/audio       # AudioContext/Analyser helpers
├─ src/hooks       # WebSocket/audio stream hook
└─ src/visualizations
```

### Quick Start (One Command)

Prerequisites:
- Rust toolchain ≥ 1.76 (install via [rustup](https://rustup.rs))
- Node.js ≥ 18
- Linux users: `sudo apt install libasound2-dev` (for `cpal`/ALSA)

```bash
git clone https://github.com/zhound420/sonicticker.git
cd sonicticker
bash scripts/bootstrap.sh
```

`bootstrap.sh` installs all dependencies, builds the Rust backend, and compiles the Vite frontend. When it finishes you’ll have:
- `oscillator/target/debug/oscillator` – Axum + tunes server
- `frontend/dist/` – production-ready SPA bundle

### Run Locally

In separate terminals:

```bash
# Terminal A – Backend (ws + REST on port 8080 by default)
cd oscillator
cargo run

# Terminal B – Frontend (Vite dev server with HMR)
cd frontend
npm run dev   # http://localhost:5173
```

### Backend

To run the Axum server manually:

```bash
cd oscillator
cargo run
```

Configuration via env vars (defaults shown):

| Variable            | Description                               | Default                                   |
|---------------------|-------------------------------------------|-------------------------------------------|
| `OSC_HOST`          | Bind host                                 | `0.0.0.0`                                 |
| `OSC_PORT`          | HTTP/WebSocket port                       | `8080`                                    |
| `OSC_BINANCE_WS`    | Binance WebSocket endpoint                | `wss://stream.binance.com:9443`          |
| `OSC_YAHOO_BASE`    | Yahoo Finance REST base URL               | `https://query1.finance.yahoo.com`       |
| `OSC_SAMPLE_RATE`   | Audio sample rate (Hz)                    | `44100`                                   |
| `OSC_CHUNK_BARS`    | Bars per generated chunk                  | `2`                                       |
| `OSC_BASE_TEMPO`    | Base BPM used by the mapper               | `104`                                     |

API surface:

- `GET /health` – liveness probe
- `GET /api/assets` – configured asset catalog
- `GET /api/metrics/:symbol` – latest indicators per asset
- `GET /ws/audio?asset=btcusdt` – bi-directional stream. Server sends alternating JSON metadata and binary audio chunks (`f32` interleaved stereo).

### Frontend

Requirements: Node 18+.

```bash
cd frontend
npm install        # already run once, rerun after dependency changes
npm run dev        # http://localhost:5173

npm run build      # production bundle in frontend/dist
```

Environment overrides (optional) – create `.env` in `frontend/`:

```
VITE_API_URL=http://localhost:8080
VITE_WS_URL=ws://localhost:8080/ws/audio
```

### Streaming Contract

1. Backend emits a JSON text frame (`AudioMetadata`) containing the next chunk’s metrics, musical parameters, timing metadata, and payload byte length.
2. Immediately following, a binary frame carries `f32` PCM samples (interleaved stereo). The frontend reconstructs `AudioBuffer`s, schedules playback to keep latency under ~150 ms, and drives the analyser nodes for visuals.

### Development Notes

- The `MarketEngine` supervises a channel per asset: Binance crypto streams (via `tokio-tungstenite`) and Yahoo polling (via `reqwest`). Each tick updates the indicator calculator (RSI, volatility, volume ratio) before mapping metrics to `tunes` composition parameters.
- Musical styles are dynamically selected based on asset class + volatility; mapper outputs tempo, harmony modes, and effect intensities. The composer renders short-burst compositions (default two bars) into PCM buffers using `tunes::Composition` + `Mixer::render_to_buffer`.
- Frontend visuals combine a particle system (beat/volume), waveform trace, and frequency bars. Asset/style selectors and the metrics dashboard live in the sidebar; start/stop and volume controls sit in the global header.

### Testing & Validation

- `cargo check` / `cargo fmt` keep the backend lint-free. Use `RUST_LOG=oscillator=debug` for verbose tracing.
- `npm run build` type-checks the client and produces the production bundle. `npm run dev` proxies WebSocket traffic to the backend during local development.
