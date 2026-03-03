# KataGo Assets - Download Instructions

These large binary files are not included in the git repository. Download them manually:

## 1. KataGo Binary (Required)

**Linux x64 (CPU/Eigen):**
```bash
cd assets/katago
curl -L -o katago.zip \
  "https://github.com/lightvector/KataGo/releases/download/v1.15.3/katago-v1.15.3-eigen-linux-x64.zip"
unzip katago.zip
chmod +x katago
rm katago.zip
```

**Size:** ~174 MB (compressed: ~40 MB)

**Verify:**
```bash
./katago version
# Should show: KataGo v1.15.3
```

## 2. Neural Network Model (Required)

**Recommended model (b10c128 - fast, small):**
```bash
cd assets/katago
curl -L -o model.bin.gz \
  "https://github.com/lightvector/KataGo/releases/download/v1.15.3/g170e-b10c128-s1141046784-d204142634.bin.gz"
```

**Size:** ~20 MB

**Alternative models:**
- Faster: `b6c96` (~10 MB) - less accurate
- Stronger: `b15c192` (~50 MB) - slower
- See: https://github.com/lightvector/KataGo/releases

## 3. Configuration Files (Already Included)

These are small text files already in git:
- ✅ `analysis.cfg` - CPU-optimized config
- ✅ `analysis_example.cfg` - Reference config
- ✅ Other example configs

## Quick Setup Script

```bash
#!/bin/bash
cd assets/katago

# Download binary
curl -L -o katago.zip \
  "https://github.com/lightvector/KataGo/releases/download/v1.15.3/katago-v1.15.3-eigen-linux-x64.zip"
unzip katago.zip
chmod +x katago
rm katago.zip

# Download model
curl -L -o model.bin.gz \
  "https://github.com/lightvector/KataGo/releases/download/v1.15.3/g170e-b10c128-s1141046784-d204142634.bin.gz"

# Test
./katago version
echo "✓ KataGo assets downloaded"
```

## System Dependencies (Arch Linux)

```bash
sudo pacman -S libzip
```

## Why Not in Git?

- Binary files (173 MB + 93 MB = 266 MB)
- GitHub limit: 100 MB per file
- Platform-specific (Linux x64)
- Should be downloaded for target platform

## Without KataGo

The game works without KataGo - territory estimation just won't be available. The backend gracefully falls back.
