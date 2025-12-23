use clap::Parser;
use std::path::Path;

use aargal::config::loader::load_config;
use aargal::engine::pipeline::process_event;
use aargal::ingest::{Ingestor, file::FileIngestor, stdin::StdinIngestor};
use aargal::model::state_store::StateStore;
use aargal::config::schema::IngestSource;
#[derive(Parser)]
#[command(name = "aargal")]
#[command(about = "Aargal — Intelligent Request Barriers for Self-Hosted Systems")]
struct Cli {
    /// Path to config file
    #[arg(short, long)]
    config: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();


    println!("Starting Aargal with config: {}", cli.config);

    let config = load_config(Path::new(&cli.config))?;

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
