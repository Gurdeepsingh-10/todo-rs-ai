#!/bin/bash

echo "Installing TODO AI..."

# Build release version
cargo build --release

# Create bin directory if doesn't exist
mkdir -p ~/.local/bin

# Copy binary
cp target/release/todo_tui ~/.local/bin/todo-ai

# Make executable
chmod +x ~/.local/bin/todo-ai

echo "Installation complete!"
echo "Run 'todo-ai' to start the application"
echo "Note: Make sure ~/.local/bin is in your PATH"