# 📄 docs/service.md

```md
# systemd Service

Running Aargal as a system service is the recommended production mode.

---

## Why systemd?

* Automatic start on boot
* Controlled restarts
* Centralized logging
* Least-privilege execution

---

## Service Model

* User: `aargal`
* Group: nginx log group (e.g. www-data)
* No root runtime
* Reads logs via group permissions

---

## Lifecycle

```bash
systemctl status aargal
systemctl start aargal
systemctl stop aargal
systemctl restart aargal
````

---

## Logs

```bash
journalctl -u aargal -f
```

---

## Restart Semantics

* Restart on failure
* Backoff handled by systemd
* No busy restart loops

---

## Config Changes

After editing `/etc/aargal/aargal.toml`:

```bash
sudo systemctl restart aargal
```

````