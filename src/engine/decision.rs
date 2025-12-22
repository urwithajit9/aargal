use crate::config::schema::{GeneralConfig, RunMode};
use crate::engine::scoring::ScoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Detect,
    Block,
}

pub fn decide(
    score: &ScoreResult,
    threshold: u32,
    general: &GeneralConfig,
) -> Decision {
    if score.score < threshold {
        return Decision::Allow;
    }

    match general.mode {
        RunMode::Detect => Decision::Detect,
        RunMode::Enforce => Decision::Block,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{GeneralConfig, RunMode};
    use crate::engine::scoring::ScoreReason;

    fn general_detect() -> GeneralConfig {
        GeneralConfig {
            mode: RunMode::Detect,
            state_ttl_seconds: 3600,
        }
    }

    fn general_enforce() -> GeneralConfig {
        GeneralConfig {
            mode: RunMode::Enforce,
            state_ttl_seconds: 3600,
        }
    }

    fn score(value: u32) -> ScoreResult {
        ScoreResult {
            score: value,
            reasons: vec![ScoreReason::HighRate { count: 100 }],
        }
    }

    #[test]
    fn allows_when_below_threshold() {
        let decision = decide(&score(50), 100, &general_detect());
        assert_eq!(decision, Decision::Allow);
    }

    #[test]
    fn detects_when_above_threshold_in_detect_mode() {
        let decision = decide(&score(120), 100, &general_detect());
        assert_eq!(decision, Decision::Detect);
    }

    #[test]
    fn blocks_when_above_threshold_in_enforce_mode() {
        let decision = decide(&score(120), 100, &general_enforce());
        assert_eq!(decision, Decision::Block);
    }
}
