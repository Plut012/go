# Features

## Implemented ✅

### Core Game
- **19×19 Go board** - Standard size, clean SVG rendering
- **Two-player multiplayer** - Real-time WebSocket synchronization
- **Capture detection** - Groups surrounded = captured
- **Ko rule enforcement** - Prevents immediate recapture
- **Pass functionality** - Both players pass = game ends
- **Reset game** - Clear board and start fresh
- **Color selection** - Choose Black or White (first come, first served)
- **Prisoner count** - Track captured stones per player
- **Turn indicator** - Visual feedback for whose turn it is

### Territory Estimation
- **KataGo integration** - Neural network analysis (v1.15.3, CPU-optimized)
- **Ownership overlay** - Subtle heat map showing territory influence
  - Black territory: dark overlay
  - White territory: light overlay
  - Contested areas: no overlay
- **Fast analysis** - ~1 second per position (50 visits)
- **Position caching** - Instant results for repeated positions
- **Graceful fallback** - Game works without KataGo if unavailable

### Deployment
- **Single binary** - One executable serves everything
- **Self-contained** - Frontend embedded, no build tools on server
- **Portable** - Copy to any Linux x64 server, just run
- **Phone access** - Works on mobile browsers (same WiFi)
- **Build scripts** - `./build.sh` and `./deploy.sh`
- **Docker support** - Optional containerized deployment
- **Small footprint** - 310MB with KataGo, 10MB without

## Planned 🚧

### Visual Themes
- **Stone evolution** - Scout → patrol → war band → battalion → legion
- **Territory transformation** - Farmland vs scorched earth
- **Army factions** - Different visual styles per theme
- **Theme selection UI** - Choose your aesthetic
- **SVG asset system** - Modular theme loading

### Board Sizes
- **9×9 board** - Quick games, learning mode
- **13×13 board** - Medium games
- **Board size selector** - UI for choosing at game start

### Polish
- **Mobile optimization** - Touch-friendly, responsive design
- **Sound effects** - Stone placement, captures (optional)
- **Game history** - View move sequence
- **Undo/redo** - Practice mode feature
- **Time controls** - Optional clock (byo-yomi, Fischer)

### Maybe Later
- **AI opponent** - Practice against KataGo (see `docs/archive/removed_feature_ai_opponent.md`)
- **Multiple games** - Concurrent game sessions
- **Persistence** - Save/load games
- **SGF export** - Download game record
- **Analysis mode** - Explore variations

## Out of Scope

- User accounts/authentication
- Chat system
- Ranking/rating system
- Matchmaking
- Game database
- Replays from database
- Teaching AI commentary

---

**Philosophy:** Ship working features. Add magic incrementally. Keep it simple and robust.
