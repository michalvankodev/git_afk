use crate::config::Configuration;
use log::{debug, info};
use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult, DebouncedEvent};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::Mutex, time::sleep};

pub struct RepositoryState {
    path: PathBuf,
    last_change_at: Instant,
}

impl RepositoryState {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
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
            let path = repo.path.clone();
            (
                path.to_str().unwrap().to_string(),
                RepositoryState::new(path),
            )
        }));

    let watch_state = Arc::new(Mutex::new(initial_state));
    // What should I do here?
    // I want to loop over the repositories but the repositories have to be always updated when anything happens.
    // Whenever a repository is added/removed from the config we should store the state and restart?
    // I will need to send the signal to the watcher that it should reload

    let state_clone = watch_state.clone();
    let repositories = state_clone.lock().await;

    repositories.values().for_each(|repo_state| {
        watch_repo(repo_state);
    });

    // Main application loop
    info!("Starting git_afk to watch repositories");
    loop {
        // Do some work

        // Sleep for 5 seconds
        sleep(Duration::from_secs(5)).await;
    }
}

fn watch_repo(repo_state: &RepositoryState) {
    debug!("Starting check for changes!");

    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        |watch_result: DebounceEventResult| match watch_result {
            Ok(events) => events
                .into_iter()
                .for_each(|event| handle_watch_event(&event)),
            Err(errors) => errors
                .iter()
                .for_each(|error| println!("notify error {error:?}")),
        },
    )
    .unwrap();

    debouncer
        .watch(&repo_state.path, RecursiveMode::Recursive)
        .unwrap();
}

fn handle_watch_event(event: &DebouncedEvent) {
    debug!("We have a notify event {:?}", event);
}
