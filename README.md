# TODO AI - Terminal Task Manager

A cross-platform, AI-powered terminal-based TODO application built with Rust.

## Features

- ✅ Terminal UI with keyboard navigation
- 🤖 AI-powered task parsing (priority & due date detection)
- 👤 Multi-user support with authentication
- 🔄 Task synchronization
- ⚙️ Customizable keybindings
- 🎨 Command history
- 🔒 Secure local storage

## Installation

### Linux/macOS
```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and install
git clone <your-repo>
cd todo-ai-rs
./install.sh
```

### Windows
```batch
# Install Rust from https://rustup.rs

# Clone and install
git clone <your-repo>
cd todo-ai-rs
install.bat
```

## Usage

### First Time Setup

1. Run the application:
```bash
   cargo run
   # or if installed: todo-ai
```

2. Register a new account:
   - Press `:` to enter register mode
   - Type: `username password email`
   - Press `Enter`

### Login

- Type: `username password`
- Press `Enter`

### Keyboard Shortcuts

**Navigation:**
- `j` / `↓` - Move down
- `k` / `↑` - Move up
- `gg` - Go to top
- `G` - Go to bottom

**Actions:**
- `space` - Toggle task done/undone
- `d` - Delete task
- `e` - Edit task
- `1/2/3` - Set priority (Low/Medium/High)
- `?` - Toggle help
- `q` - Quit

**Commands (press `:` first):**
- `:add <task>` - Add task (AI parses priority/due date)
- `:done` - Mark selected task as done
- `:sync` - Sync tasks
- `:config` - Show config location
- `:config <action> <key>` - Change keybinding
- `:quit` or `:q` - Quit

### AI Features

Add tasks with natural language:
```
:add urgent fix bug today
:add buy groceries tomorrow
:add low priority task next week
```

AI automatically detects:
- Priority: urgent/high/low keywords
- Due dates: today/tomorrow/week

## Configuration

Config file location: `~/.config/todo-ai/config.json`

Customize:
- Keybindings
- Theme colors
- AI settings

## Build from Source

### Debug build
```bash
cargo build
./target/debug/todo_tui
```

### Release build
```bash
cargo build --release
./target/release/todo_tui
```

## Requirements

- Rust 1.70+
- SQLite3
- Groq API key (optional, for AI features)

## Environment Variables

Create `.env` file:
```
GROQ_API_KEY=your_api_key_here
```

Get free API key: https://console.groq.com/keys

## Troubleshooting

**Database error:**
- Ensure `tasks.db` file has write permissions
- Check disk space

**Login issues:**
- Delete `tasks.db` and restart to reset

**Build errors:**
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`

## License

MIT

## Contributing

Pull requests welcome!
```

---

## 7. .gitignore

Create:
```
# Rust
target/
Cargo.lock

# Database
*.db
*.db-shm
*.db-wal

# Environment
.env

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Logs
*.log
