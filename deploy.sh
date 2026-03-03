#!/bin/bash
# Package everything for deployment

set -e

DEPLOY_DIR="deploy"
BINARY="target/release/go-server"

# Ensure build is done
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found. Run ./build.sh first"
    exit 1
fi

# Create deployment directory
echo "Creating deployment package..."
rm -rf $DEPLOY_DIR
mkdir -p $DEPLOY_DIR

# Copy binary
echo "  • Copying binary..."
cp $BINARY $DEPLOY_DIR/

# Copy frontend
echo "  • Copying frontend..."
cp -r frontend/dist $DEPLOY_DIR/frontend/

# Copy assets
echo "  • Copying assets..."
cp -r assets $DEPLOY_DIR/

# Create run script
cat > $DEPLOY_DIR/run.sh << 'EOF'
#!/bin/bash
# Run the Go game server
./go-server
EOF
chmod +x $DEPLOY_DIR/run.sh

# Create README
cat > $DEPLOY_DIR/README.txt << 'EOF'
Go Game Server - Deployment Package
====================================

To run:
  ./run.sh

Or directly:
  ./go-server

The server will start on http://0.0.0.0:3000

Access from:
  - Local: http://localhost:3000
  - Network: http://YOUR_IP:3000
  - Phone: http://YOUR_IP:3000 (same WiFi)

To stop: Ctrl+C

Requirements:
  - Linux x64 (or compatible)
  - libzip4 (for KataGo)
  - Port 3000 available
EOF

# Show size
SIZE=$(du -sh $DEPLOY_DIR | cut -f1)
echo
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Deployment package ready: $DEPLOY_DIR/ ($SIZE)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo
echo "To deploy to a server:"
echo "  tar -czf go-server.tar.gz $DEPLOY_DIR/"
echo "  scp go-server.tar.gz user@server:/path/"
echo "  ssh user@server"
echo "  tar -xzf go-server.tar.gz"
echo "  cd $DEPLOY_DIR && ./run.sh"
echo
