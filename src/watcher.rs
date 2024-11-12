use crate::config::Configuration;
use log::{debug, info};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::sleep;

pub struct RepositoryState {
    last_change_at: Instant,
}

impl Default for RepositoryState {
    fn default() -> Self {
        Self {
            last_change_at: Instant::now(),
        }
    }
}

pub async fn start_watcher() -> Result<(), anyhow::Error> {
    // What do wen eed to do at start???
    // Parse config file to initialize watching repositories
    let cfg: Configuration = confy::load("git_afk", None)?;

    let initial_state: HashMap<String, RepositoryState> =
        HashMap::from_iter(cfg.repositories.iter().map(|repo| {
            (
                repo.path.to_str().unwrap().to_string(),
                RepositoryState::default(),
            )
        }));

    let watch_state = Arc::new(Mutex::new(initial_state));

    // Main application loop
    info!("Starting git_afk to watch repositories");
    loop {
        // Do some work
        check_repositories().await;

        // Sleep for 5 seconds
        sleep(Duration::from_secs(5)).await;
    }
}

async fn check_repositories() {
    debug!("Starting check for changes!");
}
