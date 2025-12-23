use crate::parser::ParsedEvent;

pub trait Ingestor {
    fn next_event(&mut self) -> Option<ParsedEvent>;
}

pub mod file;
pub mod stdin;
