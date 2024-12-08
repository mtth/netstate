use clap::{Parser, Subcommand};

// We are not using Cargo's version to be able to set it programmatically more easily. See
// https://github.com/rust-lang/cargo/issues/6583 for more context.
const VERSION: &str = match option_env!("NETSTATE_VERSION") {
    Some(v) => v,
    None => "0.0.0",
};

/// Execute hooks on network state changes
#[derive(Debug, Parser)]
#[command(disable_help_subcommand = true, version = VERSION)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub enum Commands {
    /// Print current network state
    #[command(short_flag = 'Q')]
    Query,

    /// Run watcher
    #[command(short_flag = 'W')]
    Watch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let cfg = Config::parse_from(vec!["ignored", "-Q"]);
        assert_eq!(cfg.command, Commands::Query);
    }
}
