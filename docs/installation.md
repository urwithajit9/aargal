# Installation & Service Management

This section describes how to install **Aargal** on a Linux server, run it manually, or operate it as a long-running system service.

Aargal is designed to run **out-of-band**, continuously analyzing Nginx access logs and optionally signaling Fail2Ban.

---

## System Requirements

### Operating System

* Linux only
* Tested on:

  * Ubuntu 20.04 / 22.04
  * Debian 11+

### Architecture

* `x86_64` / `amd64`

Other architectures (ARM, macOS, Windows) are **not supported in Phase 1**.

### Runtime Dependencies

* Nginx (access logs enabled)
* Fail2Ban (optional, for enforcement mode)
* systemd (required for service mode)

---

## Installation Methods

### 1. Quick Install (Recommended)

Aargal provides a single installer script that:

* Detects OS & architecture
* Downloads the correct release binary
* Installs it to `/usr/local/bin`
* Optionally installs and starts a systemd service
* Installs a default configuration on first run

#### Install binary only

```bash
curl -fsSL https://raw.githubusercontent.com/urwithajit9/aargal/main/install.sh | sh
```

This installs only the `aargal` binary. No system users, configuration, or services are created.

---

#### Install and start system service immediately

```bash
curl -fsSL https://raw.githubusercontent.com/urwithajit9/aargal/main/install.sh | sudo sh -s -- --enable-service
```

This will:

* Create a system user `aargal`
* Install `/etc/systemd/system/aargal.service`
* Install `/etc/aargal/aargal.toml` (if missing)
* Start the service immediately
* Enable it at boot

---

#### Custom Nginx log group (optional)

```bash
sudo ./install.sh --enable-service --nginx-group=nginx
```

If not specified, the installer auto-detects the group from:

```text
/var/log/nginx/access.log
```

(typically `www-data`).

---

### 2. Manual Installation

If you prefer full control:

```bash
wget https://github.com/urwithajit9/aargal/releases/download/vX.Y.Z/aargal-vX.Y.Z-linux-x86_64.tar.gz
tar xzf aargal-vX.Y.Z-linux-x86_64.tar.gz
chmod +x aargal
sudo mv aargal /usr/local/bin/
```

Verify:

```bash
aargal --version
```

> Note: Manual installation does **not** create configuration files or systemd services automatically.

---

## Configuration

### Default Configuration Location

```text
/etc/aargal/aargal.toml
```

When installed as a service, this file is created automatically **only if it does not already exist**.

It is copied from the release bundle:

```text
aargal.example.toml → /etc/aargal/aargal.toml
```

Existing configurations are never overwritten.

---

### Manual Run Example

```bash
aargal --config /etc/aargal/aargal.toml
```

---

### Key Configuration Sections

* `[ingest]` – log source and polling
* `[parser]` – log format and filtering
* `[scoring]` – thresholds and weights
* `[actions]` – behavior on block
* `[fail2ban]` – socket and jail integration

Aargal does **not** modify Nginx or Fail2Ban configuration automatically.

---

## Running as a System Service

### Why systemd?

Running Aargal as a systemd service provides:

* Automatic startup on boot
* Controlled restarts
* Centralized logging via `journalctl`
* Least-privilege execution (non-root user)

---

### Service Behavior

* Runs as user: `aargal`
* Reads Nginx logs via group permissions
* Long-running daemon
* Tails `access.log`
* Restarts on failure with backoff
* Does **not** restart in a tight loop

---

### Service Control

Check status:

```bash
systemctl status aargal
```

Start / Stop:

```bash
sudo systemctl start aargal
sudo systemctl stop aargal
```

Restart:

```bash
sudo systemctl restart aargal
```

Enable on boot:

```bash
sudo systemctl enable aargal
```

---

### Logs

View live logs:

```bash
journalctl -u aargal -f
```

This includes:

* Parsing errors
* Scoring decisions
* Block actions
* Fail2Ban command output

---

## Fail2Ban Integration

### What Aargal Does

* Detects abusive behavior
* Emits Fail2Ban-compatible ban commands
* Communicates via the Fail2Ban socket

### What Aargal Does NOT Do

* Modify firewall rules
* Create Fail2Ban jails automatically
* Restart Fail2Ban

---

### Required Fail2Ban Setup

You must create a jail (example):

```ini
[aargal-auto]
enabled = true
bantime = 1h
findtime = 10m
maxretry = 1
action = iptables[name=Aargal]
```

Aargal sends ban signals to this jail.

---

## Expected Output & Verification

### Normal Operation

* Aargal silently processes logs
* No output unless thresholds are crossed

### Detect Mode

* Logs detections only
* No enforcement

### Enforce Mode

* Sends ban commands to Fail2Ban
* Logs enforcement actions

Verify impact:

```bash
fail2ban-client status aargal-auto
```

---

## Security Model

* No root runtime
* Read-only access to logs
* Explicit configuration path
* Deterministic decisions
* No network listeners

---

## Uninstall

```bash
sudo systemctl stop aargal
sudo systemctl disable aargal
sudo rm /usr/local/bin/aargal
sudo rm /etc/systemd/system/aargal.service
sudo rm -rf /etc/aargal
sudo userdel aargal
```

---

## Summary

| Feature              | Supported            |
| -------------------- | -------------------- |
| Binary install       | Yes                  |
| systemd service      | Yes                  |
| Fail2Ban integration | Yes                  |
| Hot reload           | No (restart service) |
| Auto-update          | Planned              |
| Multi-node           | Planned              |
