# Deployment Guide

Simple, robust setup that works on any server.

## Architecture

**Single Binary Design:**
- One Rust executable serves everything
- Frontend embedded (served from `frontend/dist/`)
- Assets alongside binary (`assets/` directory)
- No database, no external services
- Just run and go

---

## Local Development

**Quick start (two terminals):**

```bash
# Terminal 1: Backend
cargo run

# Terminal 2: Frontend (with hot reload)
cd frontend
npm run dev

# Open: http://localhost:5173
```

Changes to frontend auto-reload. Changes to backend require restart.

---

## Production Build

**One command:**

```bash
./build.sh
```

This creates:
- `target/release/go-server` - optimized binary (~5-10MB)
- `frontend/dist/` - static frontend files (~44KB)

**Test locally:**

```bash
./target/release/go-server

# Open: http://localhost:3000
```

---

## Deploy to Any Server

### Method 1: Simple Copy (Recommended)

**Step 1: Build**
```bash
./build.sh
```

**Step 2: Package**
```bash
./deploy.sh
# Creates deploy/ directory with everything
```

**Step 3: Copy to server**
```bash
# Option A: SCP
tar -czf go-server.tar.gz deploy/
scp go-server.tar.gz user@yourserver.com:~

# Option B: Git
git clone yourrepo.git
cd yourrepo
./build.sh
```

**Step 4: Run on server**
```bash
ssh user@yourserver.com
tar -xzf go-server.tar.gz
cd deploy
./run.sh

# Or run directly:
# ./go-server
```

**Access:**
- `http://yourserver.com:3000`
- `http://YOUR_IP:3000`

### Method 2: systemd Service (Auto-restart)

**Create service file:**
```bash
sudo nano /etc/systemd/system/go-server.service
```

```ini
[Unit]
Description=Go Game Server
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/path/to/deploy
ExecStart=/path/to/deploy/go-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Enable and start:**
```bash
sudo systemctl daemon-reload
sudo systemctl enable go-server
sudo systemctl start go-server
sudo systemctl status go-server
```

**View logs:**
```bash
sudo journalctl -u go-server -f
```

### Method 3: Docker (If Preferred)

```bash
docker build -t go-server .
docker run -p 3000:3000 go-server
```

---

## Server Requirements

**Minimal:**
- Linux x64 (or compatible)
- 512MB RAM
- 400MB disk space
- Port 3000 available

**For KataGo territory estimation:**
- 1GB RAM recommended
- libzip4 (install: `apt-get install libzip4`)
- 2+ CPU cores for faster analysis

**Without KataGo:**
- Works with just 128MB RAM
- No dependencies
- ~10MB total size

---

## Network Access

**Same machine:**
```
http://localhost:3000
```

**Local network (phones on same WiFi):**
```
http://192.168.1.135:3000
```

**Public internet:**
- Use reverse proxy (nginx)
- Or cloud provider (Railway, Fly.io)
- Or ngrok for testing

**Enable firewall:**
```bash
sudo ufw allow 3000/tcp
```

---

## Deployment Size

```
go-server (binary)        ~8MB
frontend/dist/           ~44KB
assets/katago/          ~307MB
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Total:                  ~315MB
```

**Without KataGo:** ~10MB total

---

## Updating

**On server:**
```bash
# Pull new code
git pull

# Rebuild
./build.sh

# Restart
./target/release/go-server
# Or: sudo systemctl restart go-server
```

**Zero downtime:** Run two instances on different ports, switch with nginx.

---

## Troubleshooting

**Port already in use:**
```bash
# Check what's using port 3000
sudo lsof -i :3000

# Use different port
# Edit src/main.rs line 26: change 3000 to 8080
```

**KataGo not working:**
```bash
# Test manually
./assets/katago/katago version

# If libssl error:
sudo apt-get install libzip4 libgomp1

# Disable KataGo if needed (game still works)
# Backend gracefully falls back
```

**Can't access from phone:**
- Check firewall: `sudo ufw status`
- Verify same WiFi network
- Server binds to 0.0.0.0 (all interfaces) ✓

---

## Production Checklist

- [ ] Run `./build.sh` for optimized binary
- [ ] Test locally: `./target/release/go-server`
- [ ] Copy `deploy/` directory to server
- [ ] Verify assets/ directory present
- [ ] Set up systemd service (optional)
- [ ] Configure firewall (port 3000)
- [ ] Test from phone on local network
- [ ] Set up reverse proxy for HTTPS (optional)

---

## Philosophy

**Simple. Robust. Portable.**

One binary. Copy anywhere. Run. That's it.

No build steps on server. No dependency hell. No containers required (but supported). Just a Rust executable and some files.

Works on: VPS, home server, Raspberry Pi, cloud providers, anywhere with Linux.
