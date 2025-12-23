use std::io::{self, BufRead};

use crate::parser::{parse_line, ParsedEvent};
use crate::ingest::Ingestor;



pub struct StdinIngestor {
    reader: io::StdinLock<'static>,
}

impl StdinIngestor {
    pub fn new() -> Self {
        let stdin = Box::leak(Box::new(io::stdin()));
        Self {
            reader: stdin.lock(),
        }
    }
}

impl Ingestor for StdinIngestor {
    fn next_event(&mut self) -> Option<ParsedEvent> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => parse_line(&line),
            Err(_) => None,
        }
    }
}
