# Go

A minimal, robust web-based Go game for two players.

## Quick Start

### Prerequisites
- Rust (1.75+): [Install](https://rustup.rs/)
- Node.js (18+): [Install](https://nodejs.org/)
- **Territory Estimation:** KataGo assets included (see KataGo Setup below for system dependencies)

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

## KataGo Setup (Optional - For AI Opponent)

KataGo provides AI opponent and territory estimation features. The assets are already included in `assets/katago/`, but you need to install system dependencies.

### System Dependencies

**Arch Linux:**
```bash
# Install required libraries
sudo pacman -S libzip

# Install OpenSSL 1.1 compatibility (required for pre-built binary)
yay -S openssl-1.1
# OR
paru -S openssl-1.1
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libzip4 libssl1.1
```

**macOS:**
```bash
brew install libzip openssl@1.1
```

### Verify KataGo Works

```bash
./assets/katago/katago version
```

Expected output:
```
KataGo v1.15.3
```

If you see `error while loading shared libraries: libssl.so.1.1`, install OpenSSL 1.1 (see above).

### Assets Included

The following are already in the repository:
- ✅ `assets/katago/katago` - KataGo binary (v1.15.3, CPU/Eigen, Linux x64)
- ✅ `assets/katago/analysis.cfg` - Optimized configuration for CPU
- ⚠️ `assets/katago/model.bin.gz` - Neural network (needs manual download)

### Download Neural Network Model

A neural network model is required for KataGo to function. Download a small/fast model:

```bash
cd assets/katago

# Download small 10-block network (~20MB, fast for territory estimation)
curl -L -o model.bin.gz \
  "https://github.com/lightvector/KataGo/releases/download/v1.15.3/g170e-b10c128-s1141046784-d204142634.bin.gz"
```

Or download manually from:
- Releases: https://github.com/lightvector/KataGo/releases
- Training networks: https://katagotraining.org/networks/

For this project, a small network (b10c128 or b15c192) is recommended for fast territory estimation.

### Test KataGo Integration

```bash
# Run the server (KataGo features will auto-enable if assets are present)
cargo run

# Check logs for:
# "KataGo service initialized successfully"
# OR
# "KataGo not available: <reason>" (graceful fallback)
```

### Troubleshooting

**"libssl.so.1.1: cannot open shared object file"**
- Install OpenSSL 1.1 compatibility package (see System Dependencies above)
- Alternative: Build KataGo from source with your system's OpenSSL version

**"KataGo binary not found"**
- Ensure `assets/katago/katago` exists and is executable
- Run: `chmod +x assets/katago/katago`

**"KataGo model not found"**
- Download a neural network model (see Download Neural Network Model above)
- Place it at `assets/katago/model.bin.gz`

**AI opponent not working**
- Check server logs for KataGo initialization errors
- Verify all three files exist: binary, config, model
- Test KataGo manually: `./assets/katago/katago version`

### Building KataGo from Source (Alternative)

If the pre-built binary has dependency issues, build from source:

```bash
# Install build dependencies
sudo pacman -S cmake boost zlib

# Clone and build
git clone https://github.com/lightvector/KataGo.git
cd KataGo/cpp
cmake . -DUSE_BACKEND=EIGEN
make -j$(nproc)

# Copy binary
cp katago /path/to/your/project/assets/katago/katago
```

---

## Current Status

**Production-ready** - Core game working, territory estimation integrated, deployment workflow tested.

**Features:**
- ✅ Multiplayer Go (WebSocket sync)
- ✅ KataGo territory overlay
- ✅ Single binary deployment
- ✅ Phone access (local network)
- 🚧 Visual themes (planned)
- 🚧 Board size selector (planned)

See [DEPLOY.md](../DEPLOY.md) for deployment options.
