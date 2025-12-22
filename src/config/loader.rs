use crate::config::config::Config;
use std::fs;

pub fn load(path: &str) -> Result<Config, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config: {}", e))?;

    toml::from_str(&content)
        .map_err(|e| format!("Invalid config format: {}", e))
}
