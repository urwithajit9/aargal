use std::path::Path;

use crate::config::schema::{AargalConfig, IngestSource};
use crate::doctor::report::DoctorReport;

pub fn check_ingest(
    config: &AargalConfig,
    report: &mut DoctorReport,
) -> anyhow::Result<()> {
    match config.ingest.source {
        IngestSource::File => {
            let path = &config.ingest.path;
            if path.exists() {
                report.ok(format!("Ingest file exists: {}", path.display()));
            } else {
                report.error(format!(
                    "Ingest file does not exist: {}",
                    path.display()
                ));
            }
        }
        IngestSource::Stdin => {
            report.warn("Ingest source is STDIN (intended for piping / testing)");
        }
    }
    Ok(())
}

pub fn check_fail2ban(
    config: &AargalConfig,
    report: &mut DoctorReport,
) -> anyhow::Result<()> {
    if !config.fail2ban.enabled {
        report.warn("Fail2Ban integration is disabled");
        return Ok(());
    }

    let socket = Path::new(&config.fail2ban.socket);
    if socket.exists() {
        report.ok(format!(
            "Fail2Ban socket found (jail={})",
            config.fail2ban.jail
        ));
    } else {
        report.error(format!(
            "Fail2Ban socket not found: {}",
            socket.display()
        ));
    }

    Ok(())
}

pub fn check_logging(
    config: &AargalConfig,
    report: &mut DoctorReport,
) -> anyhow::Result<()> {
    report.ok(format!(
        "Logging level set to '{}'",
        config.logging.level
    ));

    if config.logging.json {
        report.ok("JSON logging enabled");
    } else {
        report.ok("Plain-text logging enabled");
    }

    Ok(())
}
