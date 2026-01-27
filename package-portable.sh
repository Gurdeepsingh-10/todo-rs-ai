#!/bin/bash

VERSION="1.0.0"

echo "📦 Creating portable packages..."

# Build release
cargo build --release

# Linux portable
echo "Creating Linux package..."
mkdir -p todo-ai-linux-portable
cp target/release/todo_tui todo-ai-linux-portable/todo-ai
cp README.md todo-ai-linux-portable/
cat > todo-ai-linux-portable/RUN.sh << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
./todo-ai
EOF
chmod +x todo-ai-linux-portable/RUN.sh
chmod +x todo-ai-linux-portable/todo-ai
tar -czf "todo-ai-v${VERSION}-linux-x64.tar.gz" todo-ai-linux-portable
rm -rf todo-ai-linux-portable

echo "✅ Created: todo-ai-v${VERSION}-linux-x64.tar.gz"