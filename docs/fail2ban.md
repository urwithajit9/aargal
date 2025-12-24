```md
# Fail2Ban Integration

Aargal integrates with Fail2Ban via its **local socket**.

---

## Responsibility Split

| Component | Role |
|--------|------|
| Aargal | Detect & signal |
| Fail2Ban | Ban logic |
| Firewall | Enforcement |

---

## What Aargal Does

* Sends `banip` commands
* Targets a configured jail
* Does not modify firewall rules

---

## Required Jail Example

```ini
[aargal-auto]
enabled = true
bantime = 1h
findtime = 10m
maxretry = 1
````

---

## Verification

```bash
fail2ban-client status aargal-auto
```

---

## Failure Handling

* Fail2Ban down → warning only
* Detection continues

````