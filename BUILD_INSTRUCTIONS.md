# 🛠️ Build Instructions

## Prerequisites

Make sure you have installed:
- **Node.js** (v16 or later)
- **Rust** (latest stable)
- **npm** or **yarn**

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify Installation
```bash
node --version
npm --version
rustc --version
cargo --version
```

## 🚀 Quick Start

### 1. Install Dependencies
```bash
npm install
```

### 2. Run in Development Mode
```bash
npm run tauri dev
```

This will:
- Compile the Rust backend
- Start the frontend
- Open the app window (hidden by default)
- Show tray icon in menu bar

### 3. Build for Production
```bash
npm run tauri build
```

The built app will be in:
- **macOS**: `src-tauri/target/release/bundle/macos/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

## 🐛 Troubleshooting Build Issues

### "Killed: 9" Error (macOS)
This means the build ran out of memory. Try:

```bash
# Build with less parallel jobs
cargo build --manifest-path src-tauri/Cargo.toml --release -j 2
```

Or close other applications to free up memory.

### Rust Compilation Errors
```bash
# Update Rust
rustup update

# Clean and rebuild
cd src-tauri
cargo clean
cd ..
npm run tauri dev
```

### Permission Errors
```bash
# Fix npm permissions
sudo chown -R $USER ~/.npm
sudo chown -R $USER ./node_modules
```

### Missing Dependencies (Linux)
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel

# Arch
sudo pacman -S webkit2gtk \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  gtk3 \
  libappindicator-gtk3 \
  librsvg \
  libvips
```

## 📦 What Gets Built

The app includes:
- **Rust backend** (`src-tauri/src/lib.rs`) - Clipboard monitoring, database, commands
- **Frontend** (`src/`) - HTML/CSS/JS user interface
- **SQLite database** - Local clipboard storage
- **System tray icon** - Always running indicator

## 🎯 First Run

After building/running:

1. Look for the 📋 **tray icon** in your menu bar (top right on macOS)
2. Press **Cmd+Shift+V** (Mac) or **Ctrl+Shift+V** (Windows/Linux)
3. The clipboard manager window should appear
4. Start copying things - they'll be saved automatically!

## 🔍 Verify It's Working

```bash
# Check if process is running
ps aux | grep clipboard-manager

# Check database was created
ls -la clipboard_history.db
```

## 📊 Build Sizes

Approximate sizes after build:

- **Debug build**: ~200-500MB (with debug symbols)
- **Release build**: ~5-10MB (optimized)
- **Memory usage**: ~30-50MB when running
- **Database**: Grows with history (typically <10MB)

## ⚡ Development Tips

### Fast Recompilation
```bash
# Only rebuild frontend (instant)
# Edit files in src/ and they hot-reload

# Rebuild Rust backend (slower)
# Edit src-tauri/src/lib.rs and save
# The app will automatically restart
```

### Debug Logging
```bash
# Enable Rust debug logs
RUST_LOG=debug npm run tauri dev

# Open devtools in the window
# Press Cmd+Option+I (Mac) or Ctrl+Shift+I (Win/Linux)
```

### Clean Build
```bash
# Remove all build artifacts
rm -rf src-tauri/target
rm -rf node_modules
npm install
npm run tauri dev
```

## 🚀 Optimization Tips

### Faster Builds
Add to `src-tauri/Cargo.toml`:

```toml
[profile.dev]
opt-level = 1  # Some optimization in debug mode

[profile.release]
lto = true     # Link-time optimization
codegen-units = 1  # Better optimization
strip = true   # Remove debug symbols
```

### Smaller Binary
Already enabled in the config:
- LTO (Link Time Optimization)
- Strip symbols
- Optimized dependencies

## 📝 Build Checklist

Before building for release:

- [ ] Test in development mode first
- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Test all features work
- [ ] Test global shortcut
- [ ] Test system tray
- [ ] Test clipboard monitoring
- [ ] Close all other apps (for memory)
- [ ] Run `npm run tauri build`

## 🎉 Success!

If the build completes successfully, you'll have a standalone app that:

✅ Runs in the background
✅ Monitors clipboard automatically
✅ Accessible via global hotkey
✅ Never quits unless you tell it to
✅ Stores everything locally
✅ Fast and lightweight

---

**Need help?** Check `TROUBLESHOOTING.md` or the other documentation files!
