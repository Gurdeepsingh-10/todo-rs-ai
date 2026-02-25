# PLATFORM COMPATIBILITY & MULTI-DEVICE SYNC GUIDE

## Executive Summary

| Aspect | Status | Ready for Prod? |
|--------|--------|-----------------|
| **Windows Support** | Compiles ✅ | Untested ⚠️ |
| **Linux Support** | Working ✅ | Limited ⚠️ |
| **macOS Support** | Compiles ✅ | Not Signed ❌ |
| **Multi-Device Sync** | Exists ⚠️ | Conflicts ❌ |
| **Same Account Support** | Yes ✅ | Limited ⚠️ |
| **Cross-Platform Sync** | Possible ⚠️ | High Risk ❌ |

---

## PART 1: WINDOWS SUPPORT

### Current Status: ⚠️ **UNTESTED BUT BUILDS**

#### What Works ✅
- Builds release binary as `.exe`
- Build script (build.bat) functions correctly
- Installation script works (with caveats)
- Terminal UI responsive with crossterm
- All core features available

#### What's Unclear ⚠️
```
OS Versions Tested:    Unknown (assume Windows 10)
Terminal Compatibility: Likely OK (tested on Windows Terminal)
Admin Rights Handling:  Falls back to user directory
Binary Signing:        Not done (won't block, but warning shown)
```

#### What's Missing ❌
- No .msi installer (Windows convention)
- No registry entries (for PATH, uninstall)
- No Windows Store app
- No code signing certificate
- Not tested on: Windows 11, Server 2019/2022, older versions

#### Installation Path on Windows
```
Without Admin:  C:\Users\<username>\AppData\Local\Programs\TodoAI\
With Admin:     C:\Program Files\TodoAI\

If fails:       Check permissions, run as admin, or use portable version
```

#### Build Process on Windows
```batch
1. Install Rust from https://rustup.rs (adds to PATH automatically)
2. Open cmd or PowerShell in project folder
3. Run: build.bat
   - Compiles to target/debug/todo_tui.exe
   - Compiles to target/release/todo_tui.exe (optimized)
4. Run: install.bat
   - Copies release binary to installation path
   - Tries to add to PATH (may require admin)
```

#### Testing Checklist for Windows
- [ ] Windows 10 21H2 or later
- [ ] Windows 11
- [ ] Windows Server 2019/2022
- [ ] PowerShell execution (might need Set-ExecutionPolicy)
- [ ] Terminal colors (ANSI on Windows Terminal vs cmd.exe)
- [ ] Network connectivity (Supabase access)
- [ ] File permissions (config file creation)

#### Known Windows Issues to Check
1. **ANSI Colors** - May not render in Command Prompt (works in Windows Terminal)
2. **Path Addition** - Manual PATH edit often required
3. **Firewall** - May block HTTPS to Supabase
4. **First Run** - .config directory creation might fail

---

## PART 2: LINUX SUPPORT

### Current Status: ✅ **WORKING BUT LIMITED TESTING**

#### What Works ✅
- Builds debug and release binaries
- Installation to /usr/local/bin or ~/.local/bin
- Uninstall script removes binary
- Portable tar.gz creation
- Full terminal UI support
- All core features available

#### Tested Distributions
```
Likely Tested:    Ubuntu 20.04+ (implied by build.sh style)
Probably Works:   Debian 11+
Untested:         Fedora, RHEL, Arch, Gentoo, Alpine
```

#### Installation on Linux
```bash
# User installation (no sudo needed)
./install.sh
# Installs to ~/.local/bin/todo-ai

# System installation (requires sudo/admin)
# Installs to /usr/local/bin/todo-ai
```

#### Build Process
```bash
cargo build              # Debug (debug/todo_tui)
cargo build --release   # Production (release/todo_tui)
./install.sh           # Auto-builds release and installs
```

#### Platform-Specific Issues
1. **glibc version** - Binary compiled on newer glibc may not run on older systems
2. **Terminal support** - Works on GNOME Terminal, Konsole, etc.
3. **SSH sessions** - Should work fine over SSH
4. **Container support** - Would need special handling (no GUI)

#### Testing Checklist for Linux
- [ ] Ubuntu 20.04 LTS
- [ ] Ubuntu 22.04 LTS
- [ ] Fedora 37+
- [ ] Debian 11+
- [ ] Over SSH (test remote usage)
- [ ] Different terminals (bash, zsh, fish)
- [ ] File permissions
- [ ] Network connectivity

#### Known Linux Issues
1. **PATH Updates** - Script tells user to edit ~/.bashrc but doesn't do it
2. **Permissions** - May need sudo for /usr/local/bin
3. **Old glibc** - Binary may not run on CentOS 7 or very old distros

---

## PART 3: MACOS SUPPORT

### Current Status: ❌ **COMPILES BUT NOT PRODUCTION READY**

#### What Works ✅
- Builds release binary
- Creates app bundle (.app folder)
- Creates DMG package for distribution
- All core features available

#### What's Missing ❌
1. **Code Signing**
   - Currently unsigned
   - macOS shows "Unknown Developer" warning
   - Users must click "Open Anyway" to run

2. **Notarization**
   - Not submitted to Apple for verification
   - No Apple Developer account configured
   - Blocks installation on newer macOS versions

3. **Architecture Support**
   - Likely compiled for x86_64 only
   - M1/M2/M3 (ARM) Macs untested
   - May require Rosetta 2 emulation on Apple Silicon

#### Installation on macOS

**Current Process (Manual):**
```bash
./package-macos.sh
# Creates: todo-ai-macos.dmg
# User double-clicks DMG, drags app to Applications folder
```

**User Experience:**
```
1. Double-click todo-ai-macos.dmg
2. See warning: "Cannot verify developer"
3. Right-click app, choose "Open"
4. Click "Open" in dialog
5. Finally launches
```

**Better Process (Needed):**
```
1. Double-click installer
2. Click "Install"
3. App appears in Applications
4. Click to launch (no warnings)
```

#### Signing Requirements
```
To sign app:
- Need Apple Developer account ($99/year)
- Need Developer ID certificate
- Run: codesign --deep --force --verify --verbose \
       --sign "Developer ID Application" \
       TodoAI.app

To notarize:
- Submit to Apple for verification
- Wait 5-10 minutes
- Staple notarization to app
```

#### Testing Checklist for macOS
- [ ] Intel Mac (10.15+)
- [ ] Apple Silicon (M1/M2/M3)
- [ ] App bundle creation
- [ ] Code signing (if available)
- [ ] Notarization status
- [ ] Launcher (CMD+Space, type app name)
- [ ] System preferences integration
- [ ] Uninstall method

#### Known macOS Issues
1. **M1/M2 Compatibility** - May require cross-compilation
2. **Gatekeeper** - Blocks unsigned apps
3. **Privacy Permissions** - May need full disk access
4. **System Integrity Protection** - May block some file operations

---

## PART 4: MULTI-DEVICE SYNC CAPABILITY

### Current Architecture

```
Device A (Windows)          Device B (Linux)          Device C (macOS)
    ↓                           ↓                          ↓
  Local App            Local App              Local App
    ↓                           ↓                          ↓
  Task List            Task List              Task List
    ↓                           ↓                          ↓
  User: john@email.com  User: john@email.com  User: john@email.com
    ↓                           ↓                          ↓
    └─────────────────────────────────────────────────────┘
                        Supabase Database
                        (Same user ID)
```

### How Sync Currently Works

**Code Location:** [src/db/mod.rs](src/db/mod.rs) lines 103-120 (get_tasks)

```rust
pub async fn get_tasks(&self, user_id: &str) -> Result<Vec<Task>> {
    self.client
        .get(format!("{}/rest/v1/tasks", self.base_url))
        .header("apikey", &self.api_key)
        .query(&[("user_id", format!("eq.{}", user_id))])
        .query(&[("order", "position.asc")])  // Sort by position, not creation date
        .send()
        .await?
    // Returns all tasks for this user_id from Supabase
}
```

### Multi-User Same Account Support

**✅ WORKS (With Caveats):**
- Each user has unique `user_id`
- All tasks for user stored under that `user_id`
- Tasks are isolated per account
- No cross-account data leakage

**✅ Account Requirements:**
- Create account on first run: `:register john john@email.com password`
- Login on other devices: `john password`
- Auto-fetch user's tasks on login

**Example Workflow:**
```
Device A (Windows):
1. Register: :register john john@email.com mypassword
2. Add task: :add Buy milk
3. Logout (quit)

Device B (Linux):
1. Login: john mypassword
2. Sees "Buy milk" task (fetched from Supabase)
3. Edit task: e Buy milk from Costco
4. Sync: :sync

Device A (Windows):
1. Login: john mypassword
2. Run: :sync
3. Sees updated "Buy milk from Costco" task
```

### ⚠️ CRITICAL SYNC ISSUES

#### Issue #1: Last-Write-Wins Conflict

**Scenario:**
```
10:00 - Device A modifies: "Buy milk" → "Buy milk (2L)"
10:01 - Device B modifies: "Buy milk" → "Buy milk from Costco"

Timeline:
10:02 - Device A syncs
        - Sends: "Buy milk (2L)"
        - Cloud now has: "Buy milk (2L)"
        - Updates Device A ✓

10:03 - Device B syncs
        - Sends: "Buy milk from Costco"
        - Cloud now has: "Buy milk from Costco"
        - Updates Device B ✓

Result: Device A lost their change!
       "Buy milk (2L)" → overwritten by "Buy milk from Costco"
```

**Current Code:** [src/db/mod.rs](src/db/mod.rs) line 175
```rust
pub async fn update_task(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let supabase_task = json!({
        "title": task.title,
        "updated_at": Utc::now().to_rfc3339(),  // ← Only stores timestamp
        // NO version field to detect conflicts
    });
    
    self.client
        .patch(format!("{}/rest/v1/tasks", self.base_url))
        .query(&[("id", format!("eq.{}", task.id))])
        .json(&supabase_task)
        .send()
        .await?
}
```

**Why This Happens:**
- No version tracking
- No merge detection
- No conflict resolution
- Simple "last write wins"

#### Issue #2: No Offline Queue

**Scenario:**
```
User on mobile hotspot with poor connection:

10:00 - Lose internet
10:01 - Add task "Fix bug" (fails silently)
10:02 - Modify "Buy milk" (fails silently)
10:03 - Regain internet
10:04 - Sync
        → Only sees old tasks
        → Changes lost

Expected: Changes queued locally, synced when online
Actual: Changes just disappear
```

#### Issue #3: No Conflict Resolution UI

**Scenario:**
```
Conflict detected but no way to resolve:

[CONFLICT]
Device A says:  "Buy milk (2L)"
Device B says:  "Buy milk from Costco"
Server has:     "Buy milk from Costco"

Options needed:
- Keep both changes (merge)
- Choose which version
- Manual edit
- See change history

Current: Just overwrites silently
```

### What Works Across Devices ✅

- Task list fetching ✅
- Authentication same account ✅
- Task creation on any device ✅
- Task deletion on any device ✅
- Basic reordering ✅

### What Doesn't Work Across Devices ❌

- Concurrent edits to same task ❌
- Conflict detection ❌
- Conflict resolution ❌
- Offline changes queueing ❌
- Change history ❌
- Selective sync ❌

---

## PART 5: PRODUCTION READINESS MATRIX

### Platform Feature Support

| Feature | Windows | Linux | macOS |
|---------|---------|-------|-------|
| **Build** | ✅ Works | ✅ Works | ✅ Works |
| **Install** | ⚠️ Admin Issue | ✅ Good | ❌ No Installer |
| **Run** | ✅ Expected | ✅ Good | ⚠️ Unsigned |
| **Uninstall** | ✅ Works | ✅ Works | ❌ Manual |
| **Updates** | ❌ None | ❌ None | ❌ None |
| **Code Signing** | ❌ No | N/A | ❌ Critical |
| **Notarization** | N/A | N/A | ❌ Needed |

### Sync Feature Support

| Feature | Status | Issue Severity |
|---------|--------|-----------------|
| **Same Account Login** | ✅ Works | None |
| **Task Fetch Sync** | ✅ Works | None |
| **Concurrent Edits** | ❌ Broken | 🔴 CRITICAL |
| **Conflict Detection** | ❌ None | 🔴 CRITICAL |
| **Offline Mode** | ❌ Missing | 🔴 CRITICAL |
| **Offline Queue** | ❌ Missing | 🟡 HIGH |
| **Merge Strategy** | ❌ Missing | 🟡 HIGH |
| **Change History** | ❌ Missing | 🟡 HIGH |

---

## PART 6: RECOMMENDED FIXES FOR MULTI-DEVICE SYNC

### Solution 1: Version-Based Detection (Quick, Limited)

**Implementation Time:** ~10 hours  
**Effectiveness:** 40%  
**How it works:**
```rust
pub struct Task {
    version: u32,           // Increment on every change
    last_modified_device: String,  // Device that made change
    last_sync_time: DateTime,      // When synced
}

// Before updating cloud:
if local_task.version < server_task.version {
    // Server has newer version
    // Show dialog: "Task was modified elsewhere. Merge? Keep? Replace?"
}
```

**Pros:**
- Quick to implement
- Detects conflicts
- Gives user choice

**Cons:**
- Requires user intervention
- Loses some changes
- Not true synchronization

---

### Solution 2: CRDT Implementation (Recommended, Complex)

**Implementation Time:** ~30 hours  
**Effectiveness:** 95%  
**Technology:** Automerge or Yjs for Rust

```rust
// Pseudo-code
use automerge::Automerge;

pub struct SyncedTask {
    doc: Automerge,  // CRDT document
    // Automatically merges concurrent edits
}

// Device A edits title → generates change set
// Device B edits due_date → generates change set
// When synced → changes merge automatically without conflict
```

**Pros:**
- Automatic conflict resolution
- Works offline
- No user intervention needed
- True eventual consistency

**Cons:**
- More complex implementation
- Need to learn CRDT concepts
- Storage overhead

**Recommended Libraries:**
- `automerge` - Mature, feature-rich
- `yrs` - Lightweight, CRDT
- `delta-crdt` - Rust native

---

### Solution 3: Operational Transform (Medium, Complex)

**Implementation Time:** ~25 hours  
**Effectiveness:** 85%

```
Device A: Insert "milk " at position 5
Device B: Delete 4 characters at position 8

OT transforms both operations to merged result
```

**Pros:**
- Mathematically proven
- Automatic merging
- Works offline

**Cons:**
- Complex algorithm
- Not as intuitive as CRDT
- Harder to debug

---

## PART 7: ACTION PLAN

### For MVP (Current Version)

1. **Label as BETA**
   - Version: 0.1.0-beta.1
   - Add warning about sync issues
   - Document limitations

2. **Minimum Viable Sync**
   - Add version tracking
   - Show conflict warning to user
   - Implement manual merge UI

3. **Platform Support**
   - Test on Windows 10 at minimum
   - Document macOS signing requirement
   - List tested Linux distributions

### For v1.0 (Production)

1. **Implement CRDT**
   - Full automatic sync
   - Offline-first support
   - Change history

2. **Secure all Platforms**
   - Windows: Sign binary
   - macOS: Code sign + notarize
   - Linux: Verify glibc compatibility

3. **Full Testing**
   - Unit tests (50%+ coverage)
   - Integration tests (sync scenarios)
   - Platform-specific tests

---

## CONCLUSION

| Aspect | Ready Now? | When Ready? |
|--------|-----------|------------|
| Single Device Use | ✅ Yes | Now |
| Windows Usage | ⚠️ Likely | 1-2 weeks testing |
| Linux Usage | ✅ Yes | Now (Ubuntu+) |
| macOS Usage | ❌ No | 2-3 weeks signing |
| Multi-Device Sync | ❌ No | 4-6 weeks CRDT |
| **Production Release** | ❌ No | 8-12 weeks |

**Recommendation:** Release as **BETA** for single-device use while developing sync solution.
