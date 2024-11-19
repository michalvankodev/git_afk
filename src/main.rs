use clap::{arg, command, Parser};
use clap_derive::Subcommand;
use config::{add_repo, remove_repo};
use std::path::PathBuf;
use watcher::start_watcher;

mod config;
mod git;
mod watcher;

/***
    1. Setup watcher on files that are not in .gitignore
    2. Create state with timer for each repo
    3. CLI for adding repositories with configuration
    4. Timer countdown. If there are changes then commit and push according to configuration
    5. Installation into the OS as daemon
*/
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

        /// Number of seconds after which the changes are committed and pushed
        /// The timer resets on any new change
        #[arg(short, long, default_value_t = 360)]
        debounce: u64,

        /// Custom commit message that will be followed by " @ rfc2822 timestamp"
        #[arg(short, long, default_value_t = String::from("Autocommited by git_afk"))]
        msg: String,
    },

    /// Remove repositories from watch
    #[command(arg_required_else_help = true)]
    Remove {
        /// Repository to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    /// Starts a daemon process to watch all configured repositories
    Watch,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = CliArgs::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    match args.command {
        Commands::Add {
            path,
            debounce,
            msg,
        } => {
            add_repo(&path, debounce, msg)?;
        }
        Commands::Remove { path } => {
            remove_repo(&path)?;
        }
        Commands::Watch => {
            start_watcher().await?;
        }
    }
    Ok(())
}
