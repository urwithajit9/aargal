use crate::engine::scoring::ScoreResult;

pub fn log_detect(ip: &str, score: &ScoreResult) {
    log::info!(
        "AARGAL DETECT ip={} score={} reasons={:?}",
        ip,
        score.score,
        score.reasons
    );
}

pub fn log_block(ip: &str, score: &ScoreResult) {
    log::warn!(
        "AARGAL BLOCK ip={} score={} reasons={:?}",
        ip,
        score.score,
        score.reasons
    );
}
