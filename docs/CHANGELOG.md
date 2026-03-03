# Changelog

## [Unreleased] - 2026-03

### Added
- **Territory estimation** - KataGo neural network integration
  - Real-time ownership overlay after each move
  - Position caching for instant repeated queries
  - CPU-optimized (50 visits, ~1s response time)
  - Graceful fallback if KataGo unavailable
- **Production build system** - Single binary deployment
  - `./build.sh` - Build frontend + backend
  - `./deploy.sh` - Package for deployment
  - Docker support with multi-stage build
- **Deployment docs** - Complete deployment guide (DEPLOY.md)
- **Phone access** - Works on mobile browsers (local network + ngrok)

### Changed
- **Simplified scope** - Removed AI opponent feature
  - Archived at `docs/archive/removed_feature_ai_opponent.md`
  - Focused on territory visualization only
  - Reduced KataGo config from 100 to 50 visits (faster)

### Technical
- KataGo service with async ownership queries
- WebSocket protocol extended with ownership data
- OwnershipOverlay.svelte component for visualization
- Position hash-based caching
- Board → GTP coordinate conversion
- JSON query/response with KataGo analysis engine

## [0.1.0] - 2026-01

### Initial Implementation
- **Core game logic** - Standard Go rules (19×19)
  - Stone placement validation
  - Capture detection (liberty counting)
  - Ko rule enforcement
  - Suicide prevention
- **WebSocket sync** - Real-time multiplayer
  - Color selection (Black/White)
  - Move broadcast to all clients
  - Pass and reset functionality
- **Frontend** - Svelte + SVG rendering
  - Interactive board (click to place)
  - Prisoner count display
  - Turn indicator
  - Basic UI (color selection, reset)
- **Backend** - Rust + Axum
  - Game state management
  - WebSocket handler
  - Static file serving (frontend/dist)
- **Project structure** - Clean separation
  - src/game/ - Core rules
  - src/ws.rs - WebSocket logic
  - frontend/ - Svelte app
  - docs/ - Documentation

---

**Format:** [Semantic Versioning](https://semver.org/)
