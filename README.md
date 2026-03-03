# Go

*"Every time you place a stone on the board, you are exposing something of yourself."*
— Takeo Kajiwara

## What This Is

A web-based Go game with real-time territory estimation. Two players connect, place stones, and watch influence flow across the board through subtle visual overlays powered by KataGo neural network analysis.

Standard rules. Clean design. Machine learning that helps you see the game's hidden structure.

## Features

**Core Game:**
- Standard 19×19 Go (9×9 and 13×13 coming)
- Real-time WebSocket synchronization
- Capture detection and ko rule enforcement
- Pass/reset functionality

**Territory Overlay:**
- KataGo-powered ownership estimation
- Subtle heat map visualization (black/white influence)
- ~1 second analysis per position (CPU-optimized)
- Position caching for instant repeated queries

**Architecture:**
- Single Rust binary serves everything
- Svelte frontend with hot reload (dev mode)
- Zero runtime dependencies
- Works on phones (same WiFi or via ngrok)

## Philosophy

Simple. Robust. Concise. Clean. Decoupled.

Every line earns its place. The backend knows the rules. The frontend knows how things look. KataGo sees what humans can't. One binary deploys anywhere.

## Quick Start

**Development:**
```bash
cargo run              # Terminal 1: backend
cd frontend && npm run dev  # Terminal 2: frontend
# Open http://localhost:5173
```

**Production:**
```bash
./build.sh             # Build everything
./target/release/go-server  # Run
# Open http://localhost:3000
```

**Deploy:**
```bash
./deploy.sh            # Package for deployment
# Copy deploy/ to any server, run ./run.sh
```

See [DEPLOY.md](DEPLOY.md) for complete deployment guide.

## Tech Stack

- **Rust + Axum** - Game logic, WebSocket, static serving
- **Svelte + Vite** - Reactive UI with hot reload
- **KataGo** - Neural network territory estimation
- **SVG** - Scalable board rendering

Single binary. 310MB deployed (or 10MB without KataGo). No database.

## Status

**Working:** Core game, WebSocket sync, territory estimation, deployment workflow
**Next:** Visual themes (scouts → legions evolution), mobile optimization
**Future:** Theme selection, army factions, board size selector

