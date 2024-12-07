use anyhow::Result;
use is_executable::IsExecutable;
use std::env;
use std::path::PathBuf;
use std::process;
use zbus::blocking::Connection;

pub mod config;
mod manager;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn query_state() -> Result<()> {
    todo!("Query subcommand will be available soon");
}

pub fn watch_state() -> Result<()> {
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
        tracing::info!("Watching for state changes...");
        let mut online_state_stream = self.0.receive_online_state_changed();
        while let Some(msg) = online_state_stream.next() {
            let state = msg.get().expect("Unparseable state");
            tracing::info!("Running hooks with state {}.", state);
            let hooks = Hook::find_all().expect("Unloadable hooks");
            let args = vec![state.as_str()];
            for hook in hooks {
                match hook.execute(&args) {
                    Ok(status) => {
                        if status.success() {
                            tracing::info!("{:?} succeeded.", hook)
                        } else {
                            tracing::warn!("{:?} failed with status {:?}.", hook, status)
                        }
                    }
                    Err(err) => tracing::error!("{:?} errored: {}", hook, err),
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
        tracing::debug!("Found {} hook(s).", hooks.len());
        Ok(hooks)
    }

    fn execute(&self, args: &Vec<&str>) -> Result<process::ExitStatus> {
        let mut child = process::Command::new(&self.0);
        if let Ok(dir) = env::var("RUNTIME_DIRECTORY") {
            child.current_dir(dir);
        }
        Ok(child.args(args).spawn()?.wait()?)
    }
}
