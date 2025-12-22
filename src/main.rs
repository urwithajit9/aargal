use clap::Parser;

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

    Ok(())
}
