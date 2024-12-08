use anyhow::Result;
use clap::Parser;
use netstate::config::{Commands, Config};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cfg = Config::parse();
    match cfg.command {
        Commands::Query => netstate::query_state(),
        Commands::Watch => netstate::watch_state(),
    }
}
