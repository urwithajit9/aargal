use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;


use crate::parser::{parse_line, ParsedEvent};
use crate::ingest::Ingestor;

pub struct FileIngestor {
    reader: BufReader<File>,
    poll_interval: Duration,
}

impl FileIngestor {
    pub fn new(path: PathBuf, poll_interval_ms: u64) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::End(0))?;

        Ok(Self {
            reader: BufReader::new(file),
            poll_interval: Duration::from_millis(poll_interval_ms),
        })
    }
}

impl Ingestor for FileIngestor {
    fn next_event(&mut self) -> Option<ParsedEvent> {
        let mut line = String::new();

        match self.reader.read_line(&mut line) {
            Ok(0) => {
                thread::sleep(self.poll_interval);
                None
            }
            Ok(_) => parse_line(&line),
            Err(_) => None,
        }
    }
}
