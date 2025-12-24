pub mod checks;
pub mod report;

use std::path::Path;

use crate::config::loader::load_config;
use crate::config::schema::AargalConfig;

use checks::*;
use report::DoctorReport;

pub fn run_doctor(config_path: &Path) -> anyhow::Result<()> {
    let mut report = DoctorReport::new();

    let config: AargalConfig = load_config(config_path)?;
    report.ok("Config file loaded successfully");

    check_ingest(&config, &mut report)?;
    check_fail2ban(&config, &mut report)?;
    check_logging(&config, &mut report)?;

    report.print();
    Ok(())
}
