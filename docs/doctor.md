# 📄 docs/doctor.md

```md
# aargal doctor

`aargal doctor` is a **diagnostic command**.

It validates system readiness without starting the daemon.

---

## What It Checks

* Config file validity
* Log file existence & permissions
* Fail2Ban socket availability
* systemd presence
* Effective user & groups

---

## Usage

```bash
aargal doctor --config /etc/aargal/aargal.toml
````

---

## Output Levels

| Status | Meaning         |
| ------ | --------------- |
| OK     | Ready           |
| WARN   | Non-fatal issue |
| FAIL   | Will not run    |

---

## Example

```
[OK] Config loaded
[OK] Access log readable
[WARN] Fail2Ban socket not found
```

---

## When to Run

* After installation
* Before enabling service
* After config changes

````