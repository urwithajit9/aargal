use crate::config::schema::AppConfig;
use crate::engine::decision::{decide, Decision};
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
    config: &AppConfig,
) -> Result<(), PipelineError> {
    /*
     * STEP 1 — Update IP state
     */
    let ip_state = state
        .update(&event)
        .map_err(|_| PipelineError::StateUpdate)?;

    /*
     * STEP 2 — Score behavior
     */
    let score: ScoreResult =
        score_ip(&ip_state, &config.scoring);

    /*
     * STEP 3 — Make decision
     */
    let decision: Decision =
        decide(&score, &config.general);

    /*
     * STEP 4 — Execute action (side-effects only here)
     */
    execute_action(
        decision,
        &ip_state.ip,
        &score,
        &config,
    )
    .map_err(|_| PipelineError::Action)?;

    Ok(())
}
