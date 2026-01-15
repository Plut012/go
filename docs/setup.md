# Go

A minimal, robust web-based Go game for two players.

## Quick Start

### Prerequisites
- Rust (1.75+): [Install](https://rustup.rs/)
- Node.js (18+): [Install](https://nodejs.org/)

### Development

```bash
# Terminal 1: Run backend
cargo run

# Terminal 2: Run frontend dev server
cd frontend
npm install
npm run dev

# Open http://localhost:5173 (Vite dev server with HMR)
```

### Production Build

```bash
# Build frontend
cd frontend
npm install
npm run build
cd ..

# Build and run backend (serves frontend from dist/)
cargo build --release
./target/release/go-server

# Open http://localhost:3000
```

### Deploy

Run the binary on a server and share the URL:

```bash
# On VPS or local machine
./target/release/go-server

# Share with friend:
# http://your-ip:3000
# Or use ngrok: ngrok http 3000
```

## Project Structure

```
├── src/              # Rust backend (game logic, WebSocket, static serving)
├── frontend/         # Svelte frontend (UI, board rendering, theme loading)
├── themes/           # Theme assets (SVG pieces, CSS, manifest)
└── docs/             # Documentation (MVP plan, rules, philosophy)
```

## Documentation

- [MVP Plan](docs/mvp_plan.md) - Implementation roadmap
- [Overview](docs/overview.md) - Project goals and philosophy
- [Go Rules](docs/go_rules.md) - Complete ruleset
- [Theme System](docs/theme.md) - Visual theming architecture
- [Zen](docs/zen.md) - The spirit of Go

## Current Status

**MVP in progress** - Core game logic and UI scaffolding in place. See `docs/mvp_plan.md` for implementation phases.
