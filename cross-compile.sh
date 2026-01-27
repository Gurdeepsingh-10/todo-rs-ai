#!/bin/bash

echo "Building for multiple platforms..."

# Linux
echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu

# Windows (requires mingw)
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

# macOS (only works on macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Building for macOS..."
    cargo build --release --target x86_64-apple-darwin
fi

echo "Done! Binaries in target/<platform>/release/"
```

---

## 9. File Structure Summary
```
todo-ai-rs/
├── Cargo.toml
├── README.md
├── .gitignore
├── .env
├── build.sh          ✅ Created
├── build.bat         ✅ Created
├── install.sh        ✅ Created
├── install.bat       ✅ Created
├── uninstall.sh      ✅ Created
├── uninstall.bat     ✅ Created
├── tasks.db
└── src/
    └── ...