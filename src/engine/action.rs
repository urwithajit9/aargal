use crate::config::schema::{ActionsConfig, BlockAction, GeneralConfig, RunMode};
use crate::engine::decision::Decision;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionResult {
    None,
    DetectOnly,
    Block(BlockAction),
}

pub fn map_decision_to_action(
    decision: Decision,
    general: &GeneralConfig,
    actions: &ActionsConfig,
) -> ActionResult {
    match decision {
        Decision::Allow => ActionResult::None,

        Decision::Detect => ActionResult::DetectOnly,

        Decision::Block => match general.mode {
            RunMode::Detect => ActionResult::DetectOnly,
            RunMode::Enforce => ActionResult::Block(actions.on_block.clone()),
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::decision::Decision;
    use crate::config::schema::{ActionsConfig, BlockAction, GeneralConfig, RunMode};

    fn enforce_cfg() -> (GeneralConfig, ActionsConfig) {
        (
            GeneralConfig {
                mode: RunMode::Enforce,
                state_ttl_seconds: 3600,
            },
            ActionsConfig { on_block: BlockAction::Log },
        )
    }

    #[test]
    fn block_in_enforce_triggers_action() {
        let (g, a) = enforce_cfg();
        let result = map_decision_to_action(Decision::Block, &g, &a);
        assert_eq!(result, ActionResult::Block(BlockAction::Log));
    }

    #[test]
    fn block_in_detect_does_not_trigger() {
        let g = GeneralConfig { mode: RunMode::Detect,state_ttl_seconds: 3600 };
        let a = ActionsConfig { on_block: BlockAction::Fail2ban };
        let result = map_decision_to_action(Decision::Block, &g, &a);
        assert_eq!(result, ActionResult::DetectOnly);
    }
}
