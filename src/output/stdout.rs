use crate::engine::scoring::ScoreResult;

pub fn print_block(ip: &str, score: &ScoreResult) {
    println!(
        "AARGAL BLOCK ip={} score={} reasons={:?}",
        ip,
        score.score,
        score.reasons
    );
}
