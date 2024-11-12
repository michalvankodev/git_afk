use std::{path::PathBuf, time::Instant};

use clap::{arg, command, Parser};
use clap_derive::Subcommand;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use watcher::start_watcher;

mod config;
mod watcher;

/***
    1. Setup watcher on files that are not in .gitignore
    2. Create state with timer for each repo
    3. CLI for adding repositories with configuration
    4. Timer countdown. If there are changes then commit and push according to configuration
    5. Installation into the OS as daemon
*/
pub struct RepositoryState {
    last_change_at: Option<Instant>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct CliArgs {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add repositories to watch
    #[command(arg_required_else_help = true)]
    Add {
        /// Repository to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    /// Starts a daemon process to watch all configured repositories
    Watch,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    match args.command {
        Commands::Add { path } => {
            error!("TODO add functionality")
        }
        Commands::Watch => {
            start_watcher().await;
        }
    }
}
