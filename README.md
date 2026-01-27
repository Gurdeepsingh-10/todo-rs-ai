# TODO AI — Terminal Task Manager

TODO AI is a cross-platform, terminal-based task manager written in Rust with AI-assisted natural-language task parsing and optional synchronization via Supabase. It provides a keyboard-driven TUI, local secure storage, customizable keybindings, and an offline fallback for AI parsing.

- Built with: Rust, ratatui/ratatui (TUI), crossterm, tokio
- AI: Groq-compatible API (optional) + offline parser fallback
- Sync: Supabase client for multi-user task syncing
- Platforms: Linux, macOS, Windows (build & packaging scripts included)

Table of Contents
- Features
- Quick Start
- Installation
  - Requirements
  - Linux / macOS
  - Windows
  - From source
- Configuration
  - Environment variables
  - Config file & keybindings
- Usage
  - Commands
  - AI examples
- Development
  - Project layout
  - Building, packaging & cross-compilation
- Troubleshooting & Tips
- Contributing
- License
- Credits

---

## Features

- Terminal UI with keyboard navigation and modes
- AI-powered natural-language parsing of task text (priority & due detection)
- Offline parsing fallback when API is not available
- Multi-user support with Supabase for cloud sync (optional)
- Local secure storage (SQLite)
- Customizable keybindings and theme configuration
- Packaging and install scripts for Linux, macOS, Windows
- Helper scripts: build, install, package, cross-compile

---

## Quick Start

1. Install Rust (if you don't have it):  
   https://rustup.rs

2. Clone the repository:
   ```bash
   git clone https://github.com/Gurdeepsingh-10/todo-rs-ai.git
   cd todo-rs-ai
   ```

3. Create a `.env` with any optional keys (see Environment variables below).

4. Build and run (debug):
   ```bash
   cargo build
   ./target/debug/todo_tui
   ```

5. Build release:
   ```bash
   cargo build --release
   ./target/release/todo_tui
   ```

Or use the provided build/package/install scripts:
- Linux/macOS: `./build.sh`, `./install.sh`
- Windows: `build.bat`, `install.bat`, `package-windows.bat`

---

## Installation

### Requirements
- Rust 1.70+ (rustup recommended)
- SQLite3 (for local DB support)
- (Optional) Groq API key for AI features
- (Optional) Supabase project + URL and key for cloud sync

### Linux / macOS
1. Build:
   ```bash
   ./build.sh
   ```
2. Install (system-wide or per-user):
   ```bash
   sudo ./install.sh
   ```
   or (user)
   ```bash
   ./install.sh
   ```

### Windows
1. Build and install via provided batch scripts:
   ```batch
   build.bat
   install.bat
   ```

---

## Configuration

Config file location:
- `~/.config/todo-ai/config.json` (default)

Example `config.json`:
```json
{
  "theme": "default",
  "keybindings": {
    "add": ":",
    "toggle_done": "d",
    "delete": "x",
    "move_up": "k",
    "move_down": "j",
    "sync": "s",
    "search": "/"
  },
  "ai": {
    "enabled": true,
    "use_api": true
  },
  "db_path": "~/.config/todo-ai/tasks.db"
}
```

You can customize:
- Theme colors
- Keybindings
- AI settings (enable/disable online parsing)
- Database location

---

## Environment variables

Create a `.env` file in the project root or set system environment variables:

- `GROQ_API_KEY` — (optional) API key for the Groq-compatible AI endpoint. When set, the app will attempt online parsing first.
- `SUPABASE_URL` — (optional for sync) Supabase project URL
- `SUPABASE_KEY` — (optional for sync) Supabase service key

Example `.env`:
```
GROQ_API_KEY=sk-xxxxxxxxxxxxx
SUPABASE_URL=https://yourproject.supabase.co
SUPABASE_KEY=eyJhbGci...
```

Notes:
- If `GROQ_API_KEY` is not set or the API call fails, the app will use an offline parser with simple heuristics for priority and due date detection.
- Supabase is optional — without it the app uses local SQLite storage only.

---

## Usage

Launch the app and use the UI with keyboard navigation. The TUI provides modes for viewing, creating, and editing tasks.

Common commands (available via the command input or keybindings):

- add <text> — Add a task. Supports natural language parsing (AI).
  Examples:
  - `:add urgent fix login bug today`
  - `:add buy groceries tomorrow`
  - `:add low priority research next week`
- done <id> — Mark task as completed
- list — Show tasks
- delete <id> — Delete a task
- sync — Sync with Supabase (if configured)
- login / logout — Supabase authentication (UI-driven)
- quit / q — Exit the application

AI features:
- The AI parser attempts to extract:
  - `title`: main task string
  - `priority`: "low" / "medium" / "high"
  - `due_days`: (relative days) -> converted to a due date
- If online API fails, offline heuristics detect keywords: `urgent`, `high`, `low`, `today`, `tomorrow`, `week`, etc.

Command examples:
```
:add urgent fix crash on save today
:add schedule dentist appointment next week
:add low buy batteries
```

---

## AI Behavior

The AI assistant is implemented in `src/ai/mod.rs`. Behavior summary:
- Online mode: uses the Groq-compatible endpoint via `GROQ_API_KEY`. The assistant sends a prompt and expects a JSON response with fields `{title, priority, due_days}`.
- Offline mode: a local heuristic parser interprets priority and due date keywords and strips them from the title.

The offline parser maps:
- Priority: "urgent" / "high" -> High; "low" -> Low; otherwise Medium
- Due dates: "today" -> now; "tomorrow" -> +1 day; "week"/"next week" -> +7 days

---

## Syncing & Storage

- Local storage: SQLite database (default `tasks.db` in project/config dir)
- Remote sync: Supabase client (`db::SupabaseClient`) in `src/db` for multi-user sync
  - Requires `SUPABASE_URL` and `SUPABASE_KEY`
  - Sync actions: upload new tasks, fetch current user's tasks, resolve basic conflicts by position/updated timestamp

Important:
- Local data remains the source if Supabase is not configured or if network errors occur.
- Login uses Supabase authentication flows if configured; local-only mode doesn't require any account.

---

## Development

Project layout (high-level):
- Cargo.toml — Rust project manifest
- src/
  - main.rs — app entrypoint, main event loop
  - ui/ — TUI components and AppState
  - ai/ — AI assistant and parsing code
  - db/ — Supabase client and local DB adapters
  - core/ — Task struct, priority enum, utilities
  - config/ — config loading & defaults
- scripts:
  - build.sh / build.bat
  - install.sh / install.bat
  - package-*.sh / package-*.bat
  - cross-compile.sh

Building:
```bash
cargo build           # debug
cargo build --release # optimized
```

Cross-compilation:
- `cross-compile.sh` builds several targets (requires appropriate toolchains and cross toolchains for Windows/macOS).
- Packaging scripts create platform-specific bundles.

Testing:
- Add unit tests in `src/*` with `#[cfg(test)]` and run:
  ```bash
  cargo test
  ```

Logging:
- The app uses `env_logger`. Set `RUST_LOG` for more verbose logs:
  ```bash
  RUST_LOG=info ./target/debug/todo_tui
  ```

---

## Troubleshooting & Tips

- Database errors:
  - Ensure `tasks.db` is writable.
  - Check permissions and disk space.
- AI parsing unreliable:
  - If you don't have an API key or the endpoint is unreachable, the app automatically falls back to offline parsing.
  - Adjust phrasing to include clear keywords: `urgent`, `today`, `tomorrow`, `week`.
- Supabase sync issues:
  - Verify `SUPABASE_URL` and `SUPABASE_KEY`.
  - Inspect logs for network errors or auth errors.
- Build errors:
  - Update Rust toolchain: `rustup update`
  - Clean and rebuild: `cargo clean && cargo build`

---

## Contributing

Contributions are welcome! Suggested workflow:
1. Fork the repo
2. Create a feature branch: `git checkout -b feat/awesome`
3. Implement and add tests
4. Open a pull request with a clear description

Guidelines:
- Keep UI and UX consistent
- Add unit tests for core parsing and sync logic
- Keep dependencies up to date and minimal

---

## Roadmap / Ideas

- Improve AI JSON schema validation and error handling
- Add more robust conflict resolution for multi-device editing
- Add import/export (CSV / JSON)
- Add plugin hooks for integrations (calendar, todo lists)
- Add optional encrypted local storage

---

## Security & Privacy

- Sensitive keys (GROQ_API_KEY, SUPABASE_KEY) must be kept out of source control. Use `.env` or OS secret storage.
- Local database `tasks.db` is stored locally; uninstall scripts do not remove user data by default.
- When using Supabase, ensure you use appropriate service role keys and policies.

---

## License

This project is licensed under the MIT License — see LICENSE for details.

---
