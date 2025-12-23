use crate::engine::action::ActionResult;
use crate::config::schema::{BlockAction};
use crate::engine::scoring::ScoreResult;
use crate::config::schema::Fail2BanConfig;
#[derive(Debug)]
pub enum ExecutorError {
    Io(std::io::Error),
    Fail2Ban(String),
}

impl From<std::io::Error> for ExecutorError {
    fn from(e: std::io::Error) -> Self {
        ExecutorError::Io(e)
    }
}

pub fn execute_action(
    action: ActionResult,
    ip: &str,
    score: &ScoreResult,
    fail2ban: Option<&Fail2BanConfig>,
) -> Result<(), ExecutorError> {
    match action {
        ActionResult::None => Ok(()),

        ActionResult::DetectOnly => {
            crate::output::log::log_detect(ip, score);
            Ok(())
        }

        ActionResult::Block(block_action) => match block_action {
            BlockAction::Log => {
                crate::output::log::log_block(ip, score);
                Ok(())
            }
            BlockAction::Stdout => {
                crate::output::stdout::print_block(ip, score);
                Ok(())
            }
            BlockAction::Fail2ban => {
                let cfg = fail2ban.ok_or_else(|| {
                    ExecutorError::Fail2Ban("Fail2Ban config missing".into())
                })?;
                crate::output::fail2ban::ban_ip(ip, cfg)
            }
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::scoring::{ScoreResult};
    use crate::engine::scoring::ScoreReason;
    use crate::config::schema::{BlockAction};

    fn score() -> ScoreResult {
        ScoreResult {
            score: 120,
            reasons: vec![ScoreReason::HighRate { count: 50 }],
        }
    }

    #[test]
    fn allows_none_action() {
        let result = execute_action(
            ActionResult::None,
            "1.2.3.4",
            &score(),
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn allows_detect_only() {
        let result = execute_action(
            ActionResult::DetectOnly,
            "1.2.3.4",
            &score(),
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn fails_fail2ban_when_config_missing() {
        let result = execute_action(
            ActionResult::Block(BlockAction::Fail2ban),
            "1.2.3.4",
            &score(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn allows_log_block() {
        let result = execute_action(
            ActionResult::Block(BlockAction::Log),
            "1.2.3.4",
            &score(),
            None,
        );
        assert!(result.is_ok());
    }
}
