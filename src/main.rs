use anyhow::Result;
use clap::{Parser, Subcommand};
use is_executable::IsExecutable;
use std::path::PathBuf;
use std::process;
use zbus::blocking::Connection;

mod manager;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

// We are not using Cargo's version to be able to set it programmatically more easily. See
// https://github.com/rust-lang/cargo/issues/6583 for more context.
const VERSION: &str = match option_env!("NETSTATE_VERSION") {
    Some(v) => v,
    None => "0.0.0",
};

/// Execute hooks on network state changes
#[derive(Parser)]
#[command(disable_help_subcommand = true, version = VERSION)]
struct Config {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print current network state
    #[command(short_flag = 'Q')]
    Query,

    /// Run watcher
    #[command(short_flag = 'W')]
    Watch,
}

fn main() -> Result<()> {
    let cfg = Config::parse();
    match cfg.command {
        Commands::Query => query_state(),
        Commands::Watch => watch_state(),
    }
}

fn query_state() -> Result<()> {
    todo!("Query subcommand will be available soon");
}

fn watch_state() -> Result<()> {
    let watcher = Watcher::system()?;
    watcher.watch()
}

struct Watcher<'a>(manager::ManagerProxy<'a>);

impl Watcher<'_> {
    fn system<'a>() -> Result<Watcher<'a>> {
        let connection = Connection::system()?;
        let proxy = manager::ManagerProxy::new(&connection)?;
        Ok(Watcher(proxy))
    }

    fn watch(&self) -> ! {
        println!("Watching for online state changes...");
        let mut online_state_stream = self.0.receive_online_state_changed();
        while let Some(msg) = online_state_stream.next() {
            let state = msg.get().expect("Unparseable state");
            let hooks = Hook::find_all().expect("Unloadable hooks");
            let args = vec![state.as_str()];
            for hook in hooks {
                match hook.execute(&args) {
                    // TODO: Use logging library.
                    Ok(status) => {
                        if status.success() {
                            println!("{:?} succeeded.", hook)
                        } else {
                            println!("{:?} failed with status {:?}.", hook, status)
                        }
                    }
                    Err(err) => println!("{:?} errored: {}", hook, err),
                }
            }
        }
        // TODO: Gracefully exit on signal.
        unreachable!("Watcher ended unexpectedly");
    }
}

// An executable hook.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hook(PathBuf);

impl Hook {
    fn find_all() -> Result<Vec<Hook>> {
        let dirs = xdg::BaseDirectories::with_prefix(PKG_NAME)?;
        let mut hooks: Vec<Hook> = dirs
            .list_data_files("hooks.d")
            .iter()
            .filter_map(|p| {
                if p.is_executable() {
                    Some(Hook(p.to_path_buf()))
                } else {
                    None
                }
            })
            .collect();
        hooks.sort();
        Ok(hooks)
    }

    fn execute(&self, args: &Vec<&str>) -> Result<process::ExitStatus> {
        let mut child = process::Command::new(&self.0).args(args).spawn()?;
        Ok(child.wait()?)
    }
}
