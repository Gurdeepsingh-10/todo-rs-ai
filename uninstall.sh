#!/bin/bash

echo "🗑️  Uninstalling TODO AI..."
echo ""

REMOVED=0

# Check common install locations
if [ -f "/usr/local/bin/todo-ai" ]; then
    echo "Removing /usr/local/bin/todo-ai..."
    sudo rm /usr/local/bin/todo-ai
    REMOVED=1
fi

if [ -f "$HOME/.local/bin/todo-ai" ]; then
    echo "Removing $HOME/.local/bin/todo-ai..."
    rm "$HOME/.local/bin/todo-ai"
    REMOVED=1
fi

if [ $REMOVED -eq 0 ]; then
    echo "❌ TODO AI not found in standard locations"
    exit 1
fi

echo ""
echo "✅ Uninstall complete!"
echo ""
echo "📝 Note: Your data (tasks.db, config) was NOT removed"
echo "   Config: ~/.config/todo-ai/"
echo "   Database: Check your project directory"