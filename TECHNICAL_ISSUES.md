# TECHNICAL ISSUES & QUICK FIXES

## Quick Summary for Production Readiness

**Project Status: BETA - Not Production Ready**
- **Completion Level:** 70-75% of MVP features
- **Production Readiness:** 30% (core features work, critical systems missing)
- **Best Use Case:** Personal single-device task manager or beta testing

---

## CRITICAL BLOCKING ISSUES

### 1. **Missing Offline Mode** (P0)
**File:** [src/main.rs](src/main.rs) - Line 35-40  
**Issue:** App crashes if Supabase URL/Key missing or network unavailable
```rust
// Current - WILL CRASH:
let supabase_url = std::env::var("SUPABASE_URL")
    .expect("SUPABASE_URL must be set in .env");

// Need fallback:
let supabase_url = std::env::var("SUPABASE_URL")
    .unwrap_or_else(|_| {
        eprintln!("⚠️  No SUPABASE_URL found. Using offline mode.");
        String::new()  // Use SQLite locally
    });
```
**Impact:** Can't use app without cloud setup  
**Fix Time:** ~4 hours

### 2. **Sync Conflict Resolution** (P0)
**File:** [src/db/mod.rs](src/db/mod.rs) - Lines 175-200  
**Issue:** Last-write-wins overwrites concurrent edits
```rust
// PROBLEM: No version checking
pub async fn update_task(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let supabase_task = json!({
        "title": task.title,
        "updated_at": Utc::now().to_rfc3339(),
        // Missing: version_id, conflict_resolution_strategy
    });
}

// NEED: Version-based conflict detection
if task.version != server_task.version {
    // Handle conflict (show merge dialog)
}
```
**Impact:** Data loss when editing same task on 2 devices simultaneously  
**Fix Time:** ~20 hours (needs CRDT or OT implementation)

### 3. **No Encryption** (P0)
**File:** [src/db/mod.rs](src/db/mod.rs) - All create/update operations  
**Issue:** Tasks stored in plaintext in Supabase
```rust
// Currently storing plaintext:
"title": task.title,  // ← Should be encrypted

// AES-GCM available but unused:
use aes_gcm::Aes256Gcm;  // Already in Cargo.toml but not integrated
```
**Impact:** Security breach if Supabase compromised  
**Fix Time:** ~12 hours

---

## HIGH PRIORITY ISSUES

### 4. **Incomplete Error Handling** (P1)
**File:** [src/main.rs](src/main.rs) - Multiple locations  
**Issue:** Generic error messages don't tell users what failed
```rust
// CURRENT - BAD ERROR:
Err(_) => {
    state.command_input = "Login failed. Press : to register".to_string();
}

// SHOULD BE:
Err(e) => {
    state.command_input = match e.kind() {
        "network" => "Network error. Check connection.",
        "auth" => "Invalid username/password",
        "server" => "Server error. Try again later.",
        _ => "Unknown error"
    }.to_string();
}
```
**Impact:** Users can't debug failures  
**Fix Time:** ~6 hours

### 5. **No Input Validation** (P1)
**File:** [src/db/mod.rs](src/db/mod.rs) - register() and [src/main.rs](src/main.rs) - handle_command()  
**Issue:** No validation before sending to server
```rust
// MISSING: Validation
pub async fn register(&self, username: &str, email: &str, password: &str) {
    // Should check:
    // - username length (3-20 chars)
    // - email format (regex)
    // - password strength (8+ chars, uppercase, numbers)
    
    // Currently accepts anything
}
```
**Impact:** Server rejects with unclear messages, server load from invalid attempts  
**Fix Time:** ~4 hours

### 6. **Database Schema Not Documented** (P1)
**Issue:** No schema definition or migration files  
**Fix Time:** ~3 hours

---

## MEDIUM PRIORITY ISSUES

### 7. **No Undo/Recovery** (P2)
**File:** [src/main.rs](src/main.rs) - Delete operation (line 195)  
**Issue:** Deleted tasks cannot be recovered
```rust
KeyCode::Char('d') => {
    if let (Some(task), Some(ref sb), Some(ref user)) = (...) {
        sb.delete_task(&task.id).await  // ← Hard delete, no recovery
    }
}
```
**Solution:** Soft delete with timestamp
**Fix Time:** ~6 hours

### 8. **Large Task List Performance** (P2)
**File:** [src/ui/mod.rs](src/ui/mod.rs) - render() function  
**Issue:** Renders all tasks at once, slow with 1000+ items
```rust
let items: Vec<ListItem> = state
    .tasks
    .iter()  // ← Iterates ALL tasks every frame
    .enumerate()
    .map(|(i, task)| { ... })
    .collect();
```
**Solution:** Virtual scrolling (show only visible items)  
**Fix Time:** ~8 hours

### 9. **Command History Not Persistent** (P2)
**File:** [src/config/mod.rs](src/config/mod.rs)  
**Issue:** Command history lost when app closes
**Fix Time:** ~2 hours

### 10. **No Search/Filter** (P2)
**File:** [src/ui/mod.rs](src/ui/mod.rs) and [src/main.rs](src/main.rs)  
**Issue:** Can't find tasks in large lists
**Fix Time:** ~4 hours

---

## LOW PRIORITY ISSUES

### 11. **Windows Build Issues** (P3)
**File:** [install.bat](install.bat) - Line 9  
**Issue:** Requires admin rights without fallback
```batch
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo WARNING: Not running as Administrator
    set "INSTALL_DIR=%USERPROFILE%\AppData\Local\Programs\TodoAI"
)
```
Better implemented but still could fail if directory creation denied.

### 12. **macOS Code Signing** (P3)
**File:** [package-macos.sh](package-macos.sh)  
**Issue:** App not signed, users see security warnings
**Fix Time:** ~3 hours (after certificate obtained)

### 13. **No Logging to File** (P3)
**File:** [src/main.rs](src/main.rs) - Line 23  
**Issue:** Logs only go to stderr, no persistent logging
```rust
env_logger::init();  // ← Only writes to stdout/stderr
```
**Fix:** Add `fern` or `tracing-subscriber` with file output

### 14. **Cross-compile Script Missing Checks** (P3)
**File:** [cross-compile.sh](cross-compile.sh)  
**Issue:** Script doesn't verify toolchain availability

---

## PLATFORM-SPECIFIC ISSUES

### Windows 🪟

| Issue | Status | Fix Time |
|-------|--------|----------|
| Not tested on Windows 11, Server editions | ⚠️ Untested | 4 hours |
| Possible ANSI color issues on older terminals | ⚠️ Untested | 2 hours |
| No .msi installer | 🔴 Missing | 6 hours |
| Admin rights required for Program Files | ⚠️ Fallback exists | 1 hour |
| No Windows Subsystem for Linux (WSL) testing | 🔴 Not tested | 2 hours |

### Linux 🐧

| Issue | Status | Fix Time |
|-------|--------|----------|
| Only tested on Ubuntu/Debian | ⚠️ Untested | 3 hours |
| Fedora/RHEL compatibility unknown | 🔴 Not tested | 2 hours |
| No AppImage package | 🔴 Missing | 4 hours |
| No systemd service file | 🔴 Missing | 2 hours |

### macOS 🍎

| Issue | Status | Fix Time |
|-------|--------|----------|
| Not code-signed (security warning) | 🔴 Critical | 3 hours |
| Not notarized (Apple approval needed) | 🔴 Critical | 8 hours |
| M1/M2 ARM architecture issues | ⚠️ Untested | 4 hours |
| No .app in Applications folder | 🔴 Missing | 2 hours |

---

## CROSS-DEVICE SYNC ISSUES

### The Sync Problem (In Simple Terms)

```
Device A at 10:00: "Buy milk"
Device B at 10:01: "Buy milk from Costco"
Device A at 10:02: Syncs (sees "Buy milk from Costco" - OK)
Device B at 10:03: Syncs (overwrites with old "Buy milk" - ❌ DATA LOSS)
```

**Current Code:** [src/db/mod.rs](src/db/mod.rs) lines 175-200
```rust
// NO VERSION TRACKING - Can't detect conflicts
pub async fn update_task(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let supabase_task = json!({
        "title": task.title,
        "updated_at": Utc::now().to_rfc3339(),
        // Missing version field
    });
}
```

### Solution Architecture

**Option 1: Operational Transform (Medium Complexity)**
```
Device A edit + Device B edit = Merged result
Cost: ~20 hours, mature libraries available
```

**Option 2: CRDT - Conflict-free Replicated Data Types (Complex)**
```
Automatic merging without central server
Cost: ~30 hours, newer but more robust
Best for offline-first apps
```

**Option 3: Simple Version Vectors (Simple, Limited)**
```
Track version per device, show conflicts to user
Cost: ~10 hours, requires user intervention
```

### Recommended: Option 3 (Quick) → Option 2 (Long-term)

---

## QUICK FIXES YOU CAN DO NOW

### Fix #1: Add Better Error Messages (1 hour)
```rust
// File: src/main.rs, around line 335
// Replace generic error with:
Err(e) => {
    let msg = if e.to_string().contains("network") {
        "Network error - check your internet connection"
    } else if e.to_string().contains("401") {
        "Unauthorized - invalid credentials"
    } else if e.to_string().contains("500") {
        "Server error - try again later"
    } else {
        &format!("Error: {}", e)
    };
    state.set_status(msg.to_string());
}
```

### Fix #2: Add Input Validation (30 minutes)
```rust
// File: src/main.rs, handle_command function
// Add at top:
if parts[0] == "add" && parts.len() < 2 {
    state.set_status("Task title cannot be empty".to_string());
    return Ok(());
}
```

### Fix #3: Persist Command History (30 minutes)
```rust
// File: src/config/mod.rs
// Add to Config struct:
pub command_history: Vec<String>,

// In save() method:
// Write to ~/.config/todo-ai/history.json
```

### Fix #4: Add .env Validation (15 minutes)
```rust
// File: src/main.rs, at startup
let api_key = std::env::var("GROQ_API_KEY")
    .unwrap_or_else(|_| {
        eprintln!("⚠️  GROQ_API_KEY not found - AI features disabled");
        String::new()
    });
```

---

## TESTING CHECKLIST

### Must Test Before Release

- [ ] **Sync Test**: Edit same task on 2 devices, sync both → Check for conflicts
- [ ] **Offline Test**: Unplug network, try to add task → Should show error
- [ ] **Auth Test**: Wrong password → Should show "Invalid password"
- [ ] **Windows Test**: Run on Windows 10/11
- [ ] **macOS Test**: Run on Intel and M1 Mac
- [ ] **Linux Test**: Run on Ubuntu, Fedora, Arch
- [ ] **Large List**: Create 1000+ tasks → Check performance
- [ ] **Long Session**: Run app for 1 hour → Check for memory leaks

---

## BUDGET ESTIMATE FOR PRODUCTION

| Component | Estimated Hours | Priority |
|-----------|-----------------|----------|
| Sync conflict resolution | 20 | P0 |
| Offline mode | 8 | P0 |
| Encryption implementation | 12 | P0 |
| Error handling improvements | 6 | P1 |
| Input validation | 4 | P1 |
| Unit testing (50% coverage) | 40 | P1 |
| Integration testing | 20 | P1 |
| Platform-specific fixes | 24 | P1 |
| Documentation | 12 | P2 |
| Performance optimization | 16 | P2 |
| **TOTAL** | **162 hours** | |

**Timeline: ~4 weeks of full-time development**

---

## FINAL VERDICT

✅ **Currently Good For:**
- Personal single-device use
- Beta testing with early adopters
- Development foundation

❌ **NOT Ready For:**
- Production with multiple users
- Cross-device sync
- Enterprise deployment
- Public release

**Recommendation:** Label as BETA, implement P0 items before v1.0.0 release.
