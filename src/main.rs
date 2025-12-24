use clap::{Parser, Subcommand};
use std::path::PathBuf;

use aargal::doctor::run_doctor;

#[derive(Parser)]
#[command(name = "aargal")]
#[command(about = "Aargal — Intelligent Request Barriers for Self-Hosted Systems")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run Aargal as daemon
    Run {
        #[arg(short, long)]
        config: PathBuf,
    },
    /// Validate configuration and environment
    Doctor {
        #[arg(short, long)]
        config: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { config } => {
            aargal::run_daemon(&config)
        }
        Command::Doctor { config } => {
            run_doctor(&config)
        }
    }
}
