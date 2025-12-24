pub mod config;
pub mod model;
pub mod ingest;
pub mod engine;
pub mod output;
pub mod parser;
pub mod doctor;
// pub mod util;

use std::path::Path;

use crate::config::loader::load_config;
use crate::engine::pipeline::process_event;
use crate::ingest::{Ingestor, file::FileIngestor, stdin::StdinIngestor};
use crate::model::state_store::StateStore;
use crate::config::schema::IngestSource;

pub fn run_daemon(config_path: &Path) -> anyhow::Result<()> {
    println!("Starting Aargal with config: {}", config_path.display());

    let config = load_config(config_path)?;

    let mut state = StateStore::new(config.general.state_ttl_seconds);

    let mut ingestor: Box<dyn Ingestor> = match config.ingest.source {
        IngestSource::File => {
            let file_ingestor = FileIngestor::new(
                config.ingest.path.clone(),
                config.ingest.poll_interval_ms,
            )?;
            Box::new(file_ingestor)
        }
        IngestSource::Stdin => {
            Box::new(StdinIngestor::new())
        }
    };

    loop {
        if let Some(event) = ingestor.next_event() {
            let _ = process_event(event, &mut state, &config);
        }
    }
}
