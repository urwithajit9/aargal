use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct AargalConfig {
    pub general: GeneralConfig,
    pub ingest: IngestConfig,
    pub parser: ParserConfig,
    pub scoring: ScoringConfig,
    pub actions: ActionsConfig,
    pub fail2ban: Fail2BanConfig,
    pub logging: LoggingConfig,
}

/* ---------------- General ---------------- */

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunMode {
    Detect,
    Enforce,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub mode: RunMode,
    pub state_ttl_seconds: u64,
}

/* ---------------- Ingest ---------------- */

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IngestSource {
    File,
    Stdin,
}

#[derive(Debug, Deserialize)]
pub struct IngestConfig {
    pub source: IngestSource,
    pub path: PathBuf,
    pub poll_interval_ms: u64,
}

/* ---------------- Parser ---------------- */

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParserFormat {
    NginxCombined,
    Json,
}

#[derive(Debug, Deserialize)]
pub struct ParserConfig {
    pub format: ParserFormat,
    pub ignore_status: Vec<u16>,
}

/* ---------------- Scoring ---------------- */

#[derive(Debug, Deserialize)]
pub struct ScoringConfig {
    pub threshold: u32,
    pub weights: ScoringWeights,
}

#[derive(Debug, Deserialize)]
pub struct ScoringWeights {
    pub rate: u32,
    pub error: u32,
    pub user_agent: u32,
    pub path_entropy: u32,
}

/* ---------------- Actions ---------------- */

#[derive(Debug, Deserialize,Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BlockAction {
    Log,
    Stdout,
    Fail2ban,
}

#[derive(Debug, Deserialize)]
pub struct ActionsConfig {
    pub on_block: BlockAction,
}

/* ---------------- Fail2Ban ---------------- */

#[derive(Debug, Deserialize)]
pub struct Fail2BanConfig {
    pub enabled: bool,
    pub socket: PathBuf,
    pub jail: String,
}

/* ---------------- Logging ---------------- */

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub json: bool,
}
