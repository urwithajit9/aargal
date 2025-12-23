use crate::config::schema::AargalConfig;
use crate::engine::action::map_decision_to_action;
use crate::engine::decision::{decide};
use crate::engine::scoring::{score_ip, ScoreResult};
use crate::model::state_store::StateStore;
use crate::parser::ParsedEvent;
use crate::output::executor::execute_action;

#[derive(Debug)]
pub enum PipelineError {
    StateUpdate,
    Scoring,
    Decision,
    Action,
}

pub fn process_event(
    event: ParsedEvent,
    state: &mut StateStore,
    config: &AargalConfig,
) -> Result<(), PipelineError> {
    /*
     * STEP 1 — Update IP state
     */
    let ip_state = state
        .update(&event);


    /*
     * STEP 2 — Score behavior
     */
    let score: ScoreResult =
        score_ip(&ip_state, &config.scoring);

    /*
     * STEP 3 — Make decision
     */
    let decision = decide(
        &score,
        &config.general,
        &config.scoring,
    );

    let action = map_decision_to_action(
    decision,
    &config.general,
    &config.actions,
);

    /*
     * STEP 4 — Execute action (side-effects only here)
     */
    execute_action(
        action,
        &ip_state.ip,
        &score,
        Some(&config.fail2ban),
    )
    .map_err(|_| PipelineError::Action)?;

    Ok(())
}
