# Aargal – Local Testing Guide (End-to-End)

This document explains how to **locally test Aargal without Nginx or Fail2Ban**, how data flows through the system, and how to **observe detection and enforcement decisions**.

It summarizes the full testing session, including debugging steps, configuration pitfalls, and verification techniques.

---

## 1. What We Are Testing

Aargal is a **log-driven decision engine**.

**Pipeline overview:**

```
Log Input
   ↓
Ingest (file | stdin)
   ↓
Parser (nginx access log)
   ↓
State tracking (per IP)
   ↓
Scoring
   ↓
Decision (Detect | Block)
   ↓
Action (stdout | log | fail2ban)
```

Local testing focuses on validating **each stage independently**, without relying on:

* A running Nginx instance
* A real Fail2Ban service

---

## 2. Build and Run Prerequisites

### Build

```bash
cargo build
```

For debugging (recommended):

```bash
cargo build
./target/debug/aargal --help
```

---

## 3. Configuration for Local Testing

### Minimal Working `aargal.dev.toml`

```toml
[general]
mode = "detect"          # detect | enforce
state_ttl_seconds = 3600

[ingest]
source = "file"          # file | stdin
path = "./tests/fixtures/access.log"
poll_interval_ms = 100

[parser]
format = "nginx_combined"
ignore_status = [200, 301, 302]

[scoring]
threshold = 1

[scoring.weights]
rate = 40
error = 30
user_agent = 20
path_entropy = 10

[actions]
on_block = "stdout"      # stdout | log | fail2ban

[fail2ban]
enabled = false

[logging]
level = "trace"
json = false
```

Key points:

* `threshold = 1` ensures **fast triggering**
* `mode = detect` is safe for first tests
* `on_block = stdout` avoids system dependencies

---

## 4. Test Data (No Nginx Required)

### Sample Nginx Access Log

Create or append to:

```bash
tests/fixtures/access.log
```

Example lines:

```bash
echo '10.0.0.5 - - [10/Oct/2023:14:00:01 +0000] "GET /admin HTTP/1.1" 403 123 "-" "evil-bot/1.0"' >> tests/fixtures/access.log

echo '117.186.238.82 - - [02/Oct/2024:07:57:51 +0900] "GET /app/vendor/phpunit/phpunit/src/Util/PHP/eval-stdin.php HTTP/1.1" 404 2491 "-" "Custom-AsyncHttpClient"' >> tests/fixtures/access.log
```

This allows **offline replay testing**.

---

## 5. Running Aargal Locally

### Correct CLI Invocation

Aargal uses subcommands. This **will fail**:

```bash
./aargal --config file.toml
```

This is correct:

```bash
./target/debug/aargal run --config ./aargal.dev.toml
```

### Enable Verbose Logging

```bash
RUST_LOG=trace ./target/debug/aargal run --config ./aargal.dev.toml
```

---

## 6. Understanding Ingest Behavior (Critical)

### File Ingestor Behavior

```rust
file.seek(SeekFrom::End(0))?;
```

This means:

* Aargal **starts tailing from EOF**
* Existing lines are ignored
* Only **newly appended lines** are processed

### How to Trigger Events

After Aargal is running:

```bash
echo '10.0.0.5 - - [10/Oct/2023:14:00:02 +0000] "GET /admin HTTP/1.1" 403 123 "-" "evil-bot/1.0"' >> tests/fixtures/access.log
```

You should immediately see ingestion.

---

## 7. What Output Means (Step by Step)

### 1. Ingestion

```text
INGESTED EVENT: ParsedEvent {
  ip: "10.0.0.5",
  status: 403,
  path: "/admin",
  user_agent: Some("evil-bot/1.0")
}
```

Confirms:

* File read
* Parser working

---

### 2. State Tracking

```text
IP state in process_event():
IpState {
  request_count: 3,
  error_count: 3,
  blocked: false
}
```

Confirms:

* Per-IP memory
* Counters increasing

---

### 3. Scoring

```text
ScoreResult {
  score: 6,
  reasons: [HighRate { count: 3 }, HighErrorRate { errors: 3 }]
}
```

Confirms:

* Scoring logic works
* Weights are applied

---

### 4. Decision

```text
IP decision in process_event(): Detect
```

or (in enforce mode):

```text
IP decision in process_event(): Block
```

This is the **policy boundary**.

---

## 8. Detection Output (Detect Mode)

### Configuration

```toml
[general]
mode = "detect"
```

### Output

```text
AARGAL DETECT ip=10.0.0.5 score=6 reasons=[...]
```

Source:

```rust
log::info!(
    "AARGAL DETECT ip={} score={} reasons={:?}",
    ip, score.score, score.reasons
);
```

Detect mode:

* Logs only
* No enforcement
* Safe for learning

---

## 9. Enforcement Output (Enforce Mode)

### Configuration

```toml
[general]
mode = "enforce"

[actions]
on_block = "stdout"
```

### Output

```text
AARGAL BLOCK ip=10.0.0.5 score=2 reasons=[...]
```

This proves:

* Decision escalated to Block
* Action executor invoked

---

## 10. How Fail2Ban Enforcement Works (Conceptually)

When enabled:

```toml
[fail2ban]
enabled = true
socket = "/var/run/fail2ban/fail2ban.sock"
jail = "aargal-auto"
```

Executor code:

```rust
let cmd = format!("set {} banip {}\n", cfg.jail, ip);
stream.write_all(cmd.as_bytes())?;
```

Equivalent to:

```bash
fail2ban-client set aargal-auto banip 10.0.0.5
```

Key points:

* No log scraping
* Direct socket control
* Immediate firewall enforcement
* Stdout/log remains for observability

---

## 11. Common Pitfalls Observed

### 1. No Output Appearing

Cause:

* File ingestor tails from EOF
* Lines were written before startup

Fix:

* Append logs **after** Aargal starts

---

### 2. `--config` Flag Error

Cause:

* Missing subcommand

Fix:

```bash
aargal run --config file.toml
```

---

### 3. Fail2Ban Enabled but Action Not Set

Error:

```text
fail2ban.enabled=true but actions.on_block != fail2ban
```

Fix:

* Configuration consistency check is working correctly

---

## 12. Debugging Tools Used

* `RUST_LOG=trace`
* Inline debug prints inside `process_event`
* `doctor` command for preflight checks
* Controlled log replay via fixture files

---

## 13. Summary

| Area                  | Result          |
| --------------------- | --------------- |
| File ingest           | Working         |
| Stdin ingest          | Working         |
| Parser                | Working         |
| State tracking        | Verified        |
| Scoring               | Verified        |
| Detect mode           | Verified        |
| Enforce mode          | Verified        |
| Stdout executor       | Verified        |
| Fail2Ban socket logic | Correct & ready |

---

## 14. Recommended Next Steps

1. Add **block-once semantics** (`ip_state.blocked`)
2. Add Fail2Ban jail validation in `doctor`
3. Add a `--once` or `--replay` mode for testing
4. Add unit tests for executor routing

---

**This completes a full local test cycle of Aargal without external dependencies.**
