use crate::config::schema::{GeneralConfig, RunMode,ScoringConfig};
use crate::engine::scoring::ScoreResult;
use crate::config::schema::ScoringWeights;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Detect,
    Block,
}

pub fn decide(
    score: &ScoreResult,
    general: &GeneralConfig,
    scoring: &ScoringConfig,
) -> Decision {
    if score.score >= scoring.threshold {
        match general.mode {
            RunMode::Detect => Decision::Detect,
            RunMode::Enforce => Decision::Block,
        }
    } else {
        Decision::Allow
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

    fn dummy_weights() -> ScoringWeights {
    ScoringWeights {
        rate: 1,
        error: 1,
        user_agent: 1,
        path_entropy: 1,
    }
}


    fn scoring() -> ScoringConfig {
    ScoringConfig {
        threshold: 100,
        weights: dummy_weights(),
    }
}


    #[test]
    fn allows_when_below_threshold() {
        let decision = decide(&score(50), &general_detect(),&scoring());
        assert_eq!(decision, Decision::Allow);
    }

    #[test]
    fn detects_when_above_threshold_in_detect_mode() {
        let decision = decide(&score(120), &general_detect(),&scoring());
        assert_eq!(decision, Decision::Detect);
    }

    #[test]
    fn blocks_when_above_threshold_in_enforce_mode() {
        let decision = decide(&score(120),  &general_enforce(),&scoring());
        assert_eq!(decision, Decision::Block);
    }
}
