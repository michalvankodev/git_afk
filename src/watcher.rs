use crate::{config::Configuration, git::commit_and_push};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use log::{debug, error, info};
use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, DebouncedEvent, Debouncer};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{runtime::Handle, sync::Mutex, task, time::sleep};

pub struct RepositoryState {
    path: PathBuf,
    gitignore_matcher: Gitignore,
    debounce_time: Duration,
    last_change_at: Option<Instant>,
}

impl RepositoryState {
    fn new(path: PathBuf, gitignore_matcher: Gitignore, debounce_time: Duration) -> Self {
        Self {
            path,
            gitignore_matcher,
            debounce_time,
            // Expect every repo as _dirty_ on initialization
            last_change_at: Some(Instant::now()),
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
            let gitignore_matcher = GitignoreBuilder::new(&path).build().unwrap();
            let debounce_time = repo.debounce_time;
            (
                path.to_str().unwrap().to_string(),
                RepositoryState::new(path, gitignore_matcher, debounce_time),
            )
        }));

    let watch_state = Arc::new(Mutex::new(initial_state));
    // What should I do here?
    // I want to loop over the repositories but the repositories have to be always updated when anything happens.
    // Whenever a repository is added/removed from the config we should store the state and restart?
    // I will need to send the signal to the watcher that it should reload

    let state_clone = watch_state.clone();
    let repositories = state_clone.lock().await;

    let _debouncers = repositories
        .values()
        .map(|repo_state| watch_repo(watch_state.clone(), repo_state.path.clone()))
        .collect::<Vec<Debouncer<notify::INotifyWatcher, notify_debouncer_full::NoCache>>>();

    drop(repositories);

    // Main application loop
    info!("Starting git_afk to watch repositories");
    loop {
        // Check all watched repositories whether we want to commit and push
        debug!("Checking repositories");
        let watch_state = watch_state.clone();
        check_for_timeouts(watch_state).await;

        debug!("Waiting for another loop");
        // Sleep for 5 seconds
        sleep(Duration::from_secs(5)).await;
    }
}

fn watch_repo(
    watch_state: Arc<Mutex<HashMap<String, RepositoryState>>>,
    path: PathBuf,
) -> Debouncer<notify::INotifyWatcher, notify_debouncer_full::NoCache> {
    debug!("Starting check for changes for {:?}!", path);
    let path_clone = path.clone();
    let handle = Handle::current();

    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |watch_result: DebounceEventResult| {
            let watch_state = watch_state.clone();
            let path = path.clone();
            handle.spawn(async move {
                match watch_result {
                    Ok(events) => {
                        let mut watch_state = watch_state.lock().await;
                        let repo_state = watch_state.get_mut(path.to_str().unwrap()).unwrap();
                        events
                            .into_iter()
                            .for_each(|event| handle_watch_event(&event, repo_state));
                    }
                    Err(errors) => errors
                        .iter()
                        .for_each(|error| println!("notify error {error:?}")),
                }
            });
        },
    )
    .unwrap();

    debouncer
        .watch(&path_clone, RecursiveMode::Recursive)
        .unwrap();

    debug!("Watcher for {:?} has been initialized", &path_clone);
    debouncer
}

fn handle_watch_event(debounced_event: &DebouncedEvent, repo_state: &mut RepositoryState) {
    // Ignore events that we don't consider helpful
    let event_kind = debounced_event.event.kind;
    match event_kind {
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => (),
        _ => {
            return;
        }
    }

    debug!("We have a notify event {:?}", debounced_event);
    // Ignore when in `.gitignore`
    let event_is_not_ignored = debounced_event.paths.iter().any(|path| {
        let is_dir = path.as_path().is_dir();
        let match_path = repo_state.gitignore_matcher.matched(path, is_dir);
        !match_path.is_ignore()
    });

    if !event_is_not_ignored {
        return;
    }

    repo_state.last_change_at = Some(Instant::now());
}

async fn check_for_timeouts(watch_state: Arc<Mutex<HashMap<String, RepositoryState>>>) {
    let state = watch_state.lock().await;
    let repositories = state.values();
    repositories.for_each(|repository| {
        let state = watch_state.clone();
        let path = repository.path.clone();
        task::spawn({
            async move {
                let mut state = state.lock().await;
                let repository = state.get_mut(path.to_str().unwrap()).unwrap();
                if let Some(last_change) = repository.last_change_at {
                    let elapsed = last_change.elapsed();
                    if elapsed > repository.debounce_time {
                        let result = commit_and_push(repository.path.clone()).await;
                        if let Err(err) = result {
                            error!("Error while committing {:?}: {}", &path, err);
                        }
                        repository.last_change_at = None;
                    }
                }
            }
        });
    });
}
