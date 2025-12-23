use std::collections::HashMap;
use std::time::Duration;
use crate::parser::ParsedEvent;
use super::ip_state::IpState;

#[derive(Debug)]
pub struct StateStore {
    states: HashMap<String, IpState>,
    ttl: Duration,
}

impl StateStore {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            states: HashMap::new(),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Get or create state for IP
    pub fn get_or_create(&mut self, ip: &str) -> &mut IpState {
        self.states
            .entry(ip.to_string())
            .or_insert_with(|| IpState::new(ip.to_string()))
    }

    /// Read-only access (used by scoring / output)
    pub fn get(&self, ip: &str) -> Option<&IpState> {
        self.states.get(ip)
    }

    /// Remove expired IP states
    pub fn evict_expired(&mut self) {
        let ttl = self.ttl;
        self.states.retain(|_, state| state.age() <= ttl);
    }

    /// Mark an IP as blocked (decision already made upstream)
    pub fn mark_blocked(&mut self, ip: &str) {
        if let Some(state) = self.states.get_mut(ip) {
            state.mark_blocked();
        }
    }

    /// Total tracked IPs (for metrics / debugging)
    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn update(&mut self, event: &ParsedEvent) -> &IpState {
    let state = self.states
        .entry(event.ip.clone())
        .or_insert_with(|| IpState::new(event.ip.clone()));

    state.record(event);
    state
}
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn creates_and_retrieves_ip_state() {
        let mut store = StateStore::new(60);

        let state = store.get_or_create("1.2.3.4");
        state.record_request();

        let retrieved = store.get("1.2.3.4").unwrap();
        assert_eq!(retrieved.request_count, 1);
    }

    #[test]
    fn same_ip_returns_same_state() {
        let mut store = StateStore::new(60);

        store.get_or_create("1.2.3.4").record_request();
        store.get_or_create("1.2.3.4").record_request();

        let state = store.get("1.2.3.4").unwrap();
        assert_eq!(state.request_count, 2);
    }

    #[test]
    fn state_store_tracks_multiple_ips() {
        let mut store = StateStore::new(60);

        store.get_or_create("1.1.1.1");
        store.get_or_create("2.2.2.2");

        assert_eq!(store.len(), 2);
    }

    #[test]
    fn expired_states_are_evicted() {
        let mut store = StateStore::new(1); // 1 second TTL

        store.get_or_create("1.2.3.4");
        assert_eq!(store.len(), 1);

        sleep(Duration::from_secs(2));
        store.evict_expired();

        assert_eq!(store.len(), 0);
    }

    #[test]
    fn mark_blocked_sets_flag() {
        let mut store = StateStore::new(60);

        store.get_or_create("5.6.7.8");
        store.mark_blocked("5.6.7.8");

        let state = store.get("5.6.7.8").unwrap();
        assert!(state.blocked);
    }

    #[test]
    fn marking_unknown_ip_is_safe() {
        let mut store = StateStore::new(60);

        // Should not panic
        store.mark_blocked("9.9.9.9");
        assert!(store.is_empty());
    }
}
