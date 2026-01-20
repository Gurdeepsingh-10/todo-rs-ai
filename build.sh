#!/bin/bash

echo "Building TODO AI for Linux/macOS..."

# Debug build
cargo build

# Release build
cargo build --release

echo "Build complete!"
echo "Debug binary: target/debug/todo_tui"
echo "Release binary: target/release/todo_tui"