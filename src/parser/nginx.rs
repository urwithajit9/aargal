use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct ParsedEvent {
    pub ip: String,
    pub status: u16,
    pub path: String,
    pub user_agent: Option<String>,
    pub timestamp: SystemTime,

}

/// Minimal nginx combined log parser (Phase 1)
pub fn parse_line(line: &str) -> Option<ParsedEvent> {
    // Very conservative parsing — no regex yet
    let parts: Vec<&str> = line.split('"').collect();
    if parts.len() < 2 {
        return None;
    }

    let ip = parts[0].split_whitespace().next()?.to_string();

    let request = parts[1];
    let path = request.split_whitespace().nth(1)?.to_string();

    let tail = parts.get(2)?;
    let status: u16 = tail.split_whitespace().next()?.parse().ok()?;

    let user_agent = parts.get(5).map(|s| s.to_string());

    Some(ParsedEvent {
        ip,
        status,
        path,
        user_agent,
        timestamp: SystemTime::now(),
    })
}
