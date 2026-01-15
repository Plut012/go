#!/bin/bash
set -e

echo "Building Go game..."
echo ""

echo "→ Building frontend..."
cd frontend
npm install
npm run build
cd ..

echo "→ Building backend..."
cargo build --release

echo ""
echo "✓ Build complete!"
echo ""
echo "Run with: ./target/release/go-server"
echo "Then open http://localhost:3000"
