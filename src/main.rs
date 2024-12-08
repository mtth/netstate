use anyhow::Result;
use clap::Parser;
use netstate::config::{Commands, Config};


fn main() -> Result<()> {
    let cfg = Config::parse();
    match cfg.command {
        Commands::Query => netstate::query_state(),
        Commands::Watch => netstate::watch_state(),
    }
}
