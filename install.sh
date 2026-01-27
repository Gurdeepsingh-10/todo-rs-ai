#!/bin/bash

echo "📦 Installing TODO AI..."
echo ""

# Build release version first
echo "🔨 Building release version..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

# Determine install location
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Copy binary
echo "📋 Copying binary to $INSTALL_DIR/todo-ai..."
cp target/release/todo_tui "$INSTALL_DIR/todo-ai"
chmod +x "$INSTALL_DIR/todo-ai"

echo ""
echo "✅ Installation complete!"
echo ""
echo "📍 Installed to: $INSTALL_DIR/todo-ai"
echo ""

# Check if in PATH
if echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "🚀 Run the app with: todo-ai"
else
    echo "⚠️  $INSTALL_DIR is not in your PATH"
    echo ""
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo ""
    echo "Or run directly: $INSTALL_DIR/todo-ai"
fi

echo ""
echo "💡 First time? Create an account when you run the app!"