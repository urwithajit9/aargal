use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use super::schema::AargalConfig;
use crate::config::schema::BlockAction;


pub fn load_config(path: &Path) -> Result<AargalConfig> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {:?}", path))?;

    let config: AargalConfig = toml::from_str(&raw)
        .context("Failed to parse TOML configuration")?;

    validate(&config)?;

    Ok(config)
}

fn validate(cfg: &AargalConfig) -> anyhow::Result<()> {
    let w = &cfg.scoring.weights;

    let total_weight = w.rate + w.error + w.user_agent + w.path_entropy;
    if total_weight != 100 {
        anyhow::bail!(
            "scoring.weights must sum to 100 (got {})",
            total_weight
        );
    }

    if cfg.scoring.threshold == 0 {
        anyhow::bail!("scoring.threshold must be > 0");
    }

    if cfg.general.state_ttl_seconds < 60 {
        anyhow::bail!("general.state_ttl_seconds must be >= 60");
    }

    if cfg.fail2ban.enabled && cfg.actions.on_block != BlockAction::Fail2ban {
        anyhow::bail!(
            "fail2ban.enabled=true but actions.on_block != fail2ban"
        );
    }

    Ok(())
}

