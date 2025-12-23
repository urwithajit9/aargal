use std::time::{Duration, Instant};
use crate::parser::ParsedEvent;
#[derive(Debug, Clone)]
pub struct IpState {
    pub ip: String,

    /* Counters */
    pub request_count: u64,
    pub error_count: u64,

    /* Derived signals */
    pub last_seen: Instant,
    pub first_seen: Instant,

    /* Scoring */
    pub score: i32,

    /* Enforcement */
    pub blocked: bool,
}

impl IpState {
    pub fn new(ip: String) -> Self {
        let now = Instant::now();
        Self {
            ip,
            request_count: 0,
            error_count: 0,
            first_seen: now,
            last_seen: now,
            score: 0,
            blocked: false,
        }
    }

    pub fn record(&mut self, event: &ParsedEvent) {
        self.request_count += 1;

        self.last_seen = Instant::now();

        if event.status >= 400 {
            self.error_count += 1;
        }
    }

    #[inline]
    pub fn record_request(&mut self) {
        self.request_count += 1;
        self.last_seen = Instant::now();
    }

    #[inline]
    pub fn record_error(&mut self) {
        self.error_count += 1;
        self.last_seen = Instant::now();
    }

    #[inline]
    pub fn add_score(&mut self, delta: i32) {
        self.score += delta;
    }

    #[inline]
    pub fn mark_blocked(&mut self) {
        self.blocked = true;
    }

    #[inline]
    pub fn age(&self) -> Duration {
        Instant::now().duration_since(self.last_seen)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_state_initializes_correctly() {
        let state = IpState::new("1.2.3.4".to_string());

        assert_eq!(state.ip, "1.2.3.4");
        assert_eq!(state.request_count, 0);
        assert_eq!(state.error_count, 0);
        assert_eq!(state.score, 0);
        assert!(!state.blocked);
    }

    #[test]
    fn recording_requests_updates_counters() {
        let mut state = IpState::new("1.2.3.4".to_string());

        state.record_request();
        state.record_request();

        assert_eq!(state.request_count, 2);
        assert_eq!(state.error_count, 0);
    }

    #[test]
    fn recording_errors_updates_counters() {
        let mut state = IpState::new("1.2.3.4".to_string());

        state.record_error();

        assert_eq!(state.error_count, 1);
    }

    #[test]
    fn score_accumulates_correctly() {
        let mut state = IpState::new("1.2.3.4".to_string());

        state.add_score(10);
        state.add_score(-3);

        assert_eq!(state.score, 7);
    }

    #[test]
    fn blocking_flag_is_set() {
        let mut state = IpState::new("1.2.3.4".to_string());

        assert!(!state.blocked);
        state.mark_blocked();
        assert!(state.blocked);
    }
}

