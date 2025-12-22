use crate::config::schema::ScoringConfig;
use crate::model::ip_state::IpState;

#[derive(Debug, Clone)]
pub struct ScoreResult {
    pub score: u32,
    pub reasons: Vec<ScoreReason>,
}

#[derive(Debug, Clone)]
pub enum ScoreReason {
    HighRate { count: u64 },
    HighErrorRate { errors: u64 },
}

pub fn score_ip(state: &IpState, cfg: &ScoringConfig) -> ScoreResult {
    let mut score: u32 = 0;
    let mut reasons = Vec::new();

    // Request rate scoring
    if state.request_count > 0 {
        let rate_score = (state.request_count as u32)
            .min(cfg.weights.rate);
        score += rate_score;

        reasons.push(ScoreReason::HighRate {
            count: state.request_count,
        });
    }

    // Error scoring
    if state.error_count > 0 {
        let error_score = (state.error_count as u32)
            .min(cfg.weights.error);
        score += error_score;

        reasons.push(ScoreReason::HighErrorRate {
            errors: state.error_count,
        });
    }

    ScoreResult { score, reasons }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{ScoringConfig, ScoringWeights};
    use crate::model::ip_state::IpState;


    fn test_config() -> ScoringConfig {
        ScoringConfig {
            threshold: 100,
            weights: ScoringWeights {
                rate: 40,
                error: 30,
                user_agent: 20,     // unused in phase 1
                path_entropy: 10,   // unused in phase 1
            },
        }
    }

    fn test_state() -> IpState {
        IpState::new("1.2.3.4".to_string())
    }


    #[test]
    fn scores_request_rate() {
        let mut state = test_state();
        state.request_count = 50;

        let result = score_ip(&state, &test_config());

        assert!(result.score > 0);
        assert_eq!(result.reasons.len(), 1);
    }

    #[test]
    fn scores_error_rate() {
        let mut state = test_state();
        state.error_count = 10;

        let result = score_ip(&state, &test_config());

        assert!(result.score > 0);
        assert_eq!(result.reasons.len(), 1);
    }

    #[test]
    fn scores_multiple_signals() {
        let mut state = test_state();
        state.request_count = 30;
        state.error_count = 10;

        let result = score_ip(&state, &test_config());

        assert!(result.score > 0);
        assert_eq!(result.reasons.len(), 2);
    }

    #[test]
    fn zero_activity_scores_zero() {
        let state = test_state();

        let result = score_ip(&state, &test_config());

        assert_eq!(result.score, 0);
        assert!(result.reasons.is_empty());
    }
}
