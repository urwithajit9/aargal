# Aargal

**Deterministic, self-hosted abuse detection for Nginx logs.**

Aargal analyzes web server access logs out-of-band, scores request behavior deterministically, and emits clear, auditable decisions that can be enforced via Fail2Ban or other downstream systems.

It is designed for operators who want **control, predictability, and explainability**—without introducing inline latency, opaque ML models, or privileged runtime processes.

---

## What Aargal Is

Aargal is a **log-driven request behavior analyzer**.

It continuously reads Nginx access logs (or stdin), tracks per-client behavior over time, applies deterministic scoring rules, and decides whether an IP should be:

- **Allowed** (normal behavior)
- **Detected** (suspicious, logged but not blocked)
- **Blocked** (explicit enforcement via Fail2Ban or other actions)

Aargal does **not**:
- Sit inline in the request path
- Modify firewall rules directly
- Require root privileges to run
- Rely on probabilistic or ML-based decisions

Instead, it acts as a **reliable decision engine** that integrates cleanly with existing Linux security tooling.

---

## Key Features

- **Deterministic scoring**
  Identical inputs always produce identical outcomes. No randomness, no learning drift.

- **Fail2Ban integration**
  Delegates enforcement to Fail2Ban using its local socket interface.

- **systemd-native daemon**
  Runs cleanly as a managed service with proper lifecycle handling.

- **No root runtime**
  Executes as a dedicated unprivileged user with minimal permissions.

- **Auditable decisions**
  Every detect or block decision can be traced back to score, threshold, and rule inputs.

- **Out-of-band by design**
  Zero impact on request latency or application availability.

---

## Architecture (High Level)

Aargal operates as a continuous processing pipeline driven by log input.

```mermaid
flowchart TD
    A[Nginx access.log] --> B[Ingest Layer]
    B --> C[Parser]
    C --> D[Per-IP State Store]
    D --> E[Scoring Engine]
    E --> F[Decision Engine]

    F -->|Allow| G[No Action]
    F -->|Detect| H[Structured Logs]
    F -->|Block| I[Fail2Ban Command]

    I --> J[Fail2Ban Socket]
    J --> K[Fail2Ban Jail]
    K --> L[Firewall Ban]
````

### Architectural Principles

* **Separation of concerns**
  Detection (Aargal) and enforcement (Fail2Ban / firewall) are strictly decoupled.

* **Failure isolation**
  If Fail2Ban is unavailable, detection continues without crashing the daemon.

* **Operational transparency**
  Logs and decisions are explicit and inspectable.

For a deeper dive, see:
→ `docs/architecture.md`

---

## Quick Start

### Install

````bash
curl -fsSL https://raw.githubusercontent.com/urwithajit9/aargal/main/scripts/install.sh | sudo sh -s -- --enable-service
````

This will:

* Install the Aargal binary
* Create the `aargal` system user
* Install the systemd service
* Start the service immediately

---

### Verify System Readiness

````bash
aargal doctor --config /etc/aargal/aargal.toml
````

The `doctor` command validates:

* Configuration correctness
* Log file access permissions
* Fail2Ban socket availability
* Runtime environment assumptions

---

### Check Service Status

````bash
systemctl status aargal
````

View logs:

````bash
journalctl -u aargal -f
````

---

## Documentation

Detailed documentation is maintained under `docs/` and is structured for future conversion into a full documentation site.

## Documentation

- **Installation** → [Installation Guide](docs/installation.md)
- **Architecture** → [System Architecture](docs/architecture.md)
- **Configuration** → [Configuration Reference](docs/configuration.md)
- **Service Management** → [systemd Service Guide](docs/service.md)
- **Diagnostics (doctor)** → [Diagnostics & Health Checks](docs/doctor.md)
- **Fail2Ban Integration** → [Fail2Ban Integration](docs/fail2ban.md)
- **CI/CD & Releases** → [Builds, Releases, and CI/CD](docs/ci-cd.md)



---

## Project Status

**Phase 1 — Deterministic Core (Stable)**

* Core ingestion, parsing, scoring, and decision logic implemented
* systemd service support complete
* Fail2Ban integration stable
* `aargal doctor` available for diagnostics

Future phases will focus on:

* Extended rule sets
* Improved observability
* Optional state persistence
* Additional ingestion formats

---

## License

Aargal is dual-licensed under:

* **MIT License**
* **Apache License 2.0**

You may choose either license according to your needs.

---

*Aargal is built for operators who value clarity over cleverness and control over automation.*
