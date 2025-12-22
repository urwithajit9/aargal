use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub ingest: Ingest,
    pub parser: Parser,
    pub scoring: Scoring,
    pub actions: Actions,
    pub fail2ban: Fail2Ban,
    pub logging: Logging,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub mode: Mode,
    pub state_ttl_seconds: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Detect,
    Enforce,
}

#[derive(Debug, Deserialize)]
pub struct Ingest {
    pub source: IngestSource,
    pub path: Option<PathBuf>,
    pub poll_interval_ms: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IngestSource {
    File,
    Stdin,
}

#[derive(Debug, Deserialize)]
pub struct Parser {
    pub format: ParserFormat,
    pub ignore_status: Vec<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParserFormat {
    NginxCombined,
    Json,
}

#[derive(Debug, Deserialize)]
pub struct Scoring {
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

#[derive(Debug, Deserialize)]
pub struct Actions {
    pub on_block: BlockAction,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockAction {
    Log,
    Stdout,
    Fail2ban,
}

#[derive(Debug, Deserialize)]
pub struct Fail2Ban {
    pub enabled: bool,
    pub socket: PathBuf,
    pub jail: String,
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub level: LogLevel,
    pub json: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
