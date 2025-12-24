# 📄 docs/configuration.md

```md
# Configuration

Aargal uses a **TOML configuration file**.

Default location:
````

/etc/aargal/aargal.toml

````

---

## Minimal Example

```toml
[general]
mode = "detect"
state_ttl_seconds = 3600

[ingest]
source = "file"
path = "/var/log/nginx/access.log"
poll_interval_ms = 500

[scoring]
threshold = 100

[actions]
on_block = "log"
````

---

## Section Reference

### [general]

| Field             | Description       |
| ----------------- | ----------------- |
| mode              | detect / enforce  |
| state_ttl_seconds | IP state eviction |

---

### [ingest]

| Field            | Description    |
| ---------------- | -------------- |
| source           | file / stdin   |
| path             | log path       |
| poll_interval_ms | read frequency |

---

### [scoring]

Defines thresholds and weights.

---

### [actions]

Controls behavior on block:

* log
* stdout
* fail2ban

---

### [fail2ban]

Only required if enabled.

---

## Configuration Philosophy

* No auto-discovery
* No implicit defaults
* Explicit > magic

````