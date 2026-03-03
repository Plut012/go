#!/bin/bash
# Build script for production deployment

set -e

echo "Building Go Game Server..."
echo

# Build frontend
echo "1. Building frontend..."
cd frontend
npm install --silent
npm run build
cd ..
echo "   ✓ Frontend built to frontend/dist/"

# Build backend (release mode)
echo
echo "2. Building backend (release mode)..."
cargo build --release
echo "   ✓ Backend binary at target/release/go-server"

# Show deployment info
echo
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Build complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo
echo "To deploy, copy these to your server:"
echo "  • target/release/go-server (binary)"
echo "  • frontend/dist/ (frontend files)"
echo "  • assets/ (KataGo + neural network)"
echo
echo "Then run: ./go-server"
echo
echo "Local test: ./target/release/go-server"
echo "           (then open http://localhost:3000)"
echo
