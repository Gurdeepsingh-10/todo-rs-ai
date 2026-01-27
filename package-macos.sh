#!/bin/bash

VERSION="1.0.0"
APP_NAME="TodoAI"

echo "📦 Creating macOS app bundle..."

# Build release
cargo build --release

# Create app structure
mkdir -p "$APP_NAME.app/Contents/MacOS"
mkdir -p "$APP_NAME.app/Contents/Resources"

# Copy binary
cp target/release/todo_tui "$APP_NAME.app/Contents/MacOS/todo-ai"
chmod +x "$APP_NAME.app/Contents/MacOS/todo-ai"

# Create Info.plist
cat > "$APP_NAME.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>todo-ai</string>
    <key>CFBundleIdentifier</key>
    <string>com.yourdomain.todoai</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
</dict>
</plist>
EOF

# Create DMG
echo "Creating DMG..."
hdiutil create -volname "$APP_NAME" -srcfolder "$APP_NAME.app" -ov -format UDZO "todo-ai-macos.dmg"

echo "✅ Package created: todo-ai-macos.dmg"

# Cleanup
rm -rf "$APP_NAME.app"