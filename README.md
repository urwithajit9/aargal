# Aargal

**Aargal (आर्गल)** means *barrier*, *gate*, or *restraint* — a control point that decides what passes through.

**Aargal** is a lightweight, self-hosted, Rust-based log analysis tool for detecting abusive or scraper-like IP behavior from web server logs and feeding enforcement systems such as **Fail2Ban**.

It is designed for operators who want **control, transparency, and predictability** — without cloud lock-in or opaque ML systems.

---

## Target Users

* EC2 / VPS operators
* Self-hosted infrastructure owners
* Nginx + Fail2Ban users
* Developers who want deterministic security controls

---

## Phase 1 Goals (Validation / OSS)

Phase 1 is intentionally conservative and deterministic.

* Log-based scraper and abuse detection
* Deterministic behavior scoring (no ML)
* IP-based state tracking
* Fail2Ban-compatible output
* CLI-driven workflow
* Minimal runtime dependencies
* Fully self-hosted

This phase is intended to **build trust, validate assumptions, and harden the core logic**.

---

## Non-Goals (Phase 1)

The following are explicitly out of scope for Phase 1:

* Packet inspection
* Inline firewall manipulation
* Kernel modules
* Machine learning or AI models
* Cloud services or SaaS components
* Web UI or dashboards

---

## Architecture Overview

Aargal operates as an **out-of-band analyzer**.

1. Ingests Nginx access logs
2. Parses requests deterministically
3. Tracks per-IP behavior state
4. Scores behavior against fixed thresholds
5. Emits audit and enforcement output
6. Delegates blocking to Fail2Ban / firewall

---

## Requirements

* Linux (tested on Ubuntu 20.04 / 22.04)
* Rust 1.75+ (stable)
* Nginx access logs
* Fail2Ban (optional but recommended)

---

## Project Structure

Current structure (Phase 1, evolving):

```
aargal/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── aargal.dev.toml
└── src/
    ├── main.rs
    ├── cli/        # CLI parsing and commands
    ├── config/     # Config schema and loading
    ├── ingest/     # Log ingestion (tail / batch)
    ├── parser/     # Log format parsing
    ├── state/      # IP state & eviction
    ├── scoring/    # Behavior scoring logic
    └── output/     # Audit + Fail2Ban output
```

Each module has **single responsibility** and minimal coupling.

---

## Configuration

Aargal uses a **TOML configuration file**.

### Development configuration

Recommended path during development:

```
./aargal.dev.toml
```

Run with:

```
cargo run -- --config ./aargal.dev.toml
```

### Production configuration (planned)

```
/etc/aargal/aargal.toml
```

No auto-discovery or magic paths are used — configuration must be explicit.

---

## Build & Sanity Check

Verify the project builds correctly:

```
cargo check
```

Expected output:

```
Finished `dev` profile [unoptimized + debuginfo]
```

This represents a **known-good baseline**.

---

## Integration Model

### With Nginx

* Aargal reads existing access logs
* No Nginx module required
* No request-path interference

### With Fail2Ban

* Aargal emits Fail2Ban-compatible log entries
* Fail2Ban owns:

  * Banning
  * Escalation
  * Firewall rules

Aargal **does not modify firewall state directly** in Phase 1.

---

## Explainability & Auditability

Every enforcement decision is explainable:

* IP address
* Triggering conditions
* Scoring breakdown
* Timestamp
* Output classification

This enables:

* Post-incident audits
* False-positive analysis
* Rule tuning without guesswork

---

## License

Aargal is dual-licensed:

* **Apache License 2.0**
* **MIT License**

You may choose either.

---

## Project Status

* Phase: **1 — Validation / OSS**
* Stability: **Early, actively developed**
* API: **Unstable**
* Config schema: **Subject to change until Phase 1 lock**

---

## Guiding Principles

* Determinism over heuristics
* Explainability over automation
* OSS-first development
* Minimalism over feature sprawl


