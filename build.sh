#!/bin/bash

echo "🔨 Building TODO AI for Linux/macOS..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs"
    exit 1
fi

echo "📦 Building debug version..."
cargo build
if [ $? -ne 0 ]; then
    echo "❌ Debug build failed"
    exit 1
fi

echo ""
echo "📦 Building release version (optimized)..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Release build failed"
    exit 1
fi

echo ""
echo "✅ Build complete!"
echo ""
echo "📍 Binaries location:"
echo "   Debug:   $(pwd)/target/debug/todo_tui"
echo "   Release: $(pwd)/target/release/todo_tui"
echo ""
echo "🚀 Run debug:   ./target/debug/todo_tui"
echo "🚀 Run release: ./target/release/todo_tui"
echo ""
echo "💡 To install system-wide, run: ./install.sh"