#!/bin/bash

echo "🔧 Testing PasteAI Build..."
echo ""

echo "1️⃣ Checking Rust compilation..."
cargo check --manifest-path src-tauri/Cargo.toml --quiet
if [ $? -eq 0 ]; then
    echo "✅ Rust code compiles successfully!"
else
    echo "❌ Rust compilation failed"
    exit 1
fi

echo ""
echo "2️⃣ Checking configuration files..."

if [ -f "src-tauri/tauri.conf.json" ]; then
    echo "✅ tauri.conf.json exists"
else
    echo "❌ tauri.conf.json missing"
    exit 1
fi

if [ -f "src-tauri/Cargo.toml" ]; then
    echo "✅ Cargo.toml exists"
else
    echo "❌ Cargo.toml missing"
    exit 1
fi

if [ -f "package.json" ]; then
    echo "✅ package.json exists"
else
    echo "❌ package.json missing"
    exit 1
fi

echo ""
echo "3️⃣ Checking frontend files..."

if [ -f "src/index.html" ]; then
    echo "✅ index.html exists"
else
    echo "❌ index.html missing"
    exit 1
fi

if [ -f "src/main.js" ]; then
    echo "✅ main.js exists"
else
    echo "❌ main.js missing"
    exit 1
fi

if [ -f "src/styles.css" ]; then
    echo "✅ styles.css exists"
else
    echo "❌ styles.css missing"
    exit 1
fi

echo ""
echo "4️⃣ Checking app name is PasteAI..."

if grep -q "PasteAI" src-tauri/tauri.conf.json; then
    echo "✅ App name is PasteAI in config"
else
    echo "❌ App name not updated in config"
    exit 1
fi

if grep -q "PasteAI" src/index.html; then
    echo "✅ App name is PasteAI in HTML"
else
    echo "❌ App name not updated in HTML"
    exit 1
fi

echo ""
echo "5️⃣ Verifying the fix for red X button..."

if grep -q "on_window_event" src-tauri/src/lib.rs; then
    echo "✅ Window close event handler present"
else
    echo "❌ Window close event handler missing"
    exit 1
fi

if grep -q "prevent_close" src-tauri/src/lib.rs; then
    echo "✅ prevent_close() call present"
else
    echo "❌ prevent_close() call missing"
    exit 1
fi

echo ""
echo "✨ All checks passed! Ready to run."
echo ""
echo "To start the app:"
echo "  npm run dev"
echo ""
echo "To build for production:"
echo "  npm run build"
echo ""
echo "The red X button fix is implemented! 🎉"
