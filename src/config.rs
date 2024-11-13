use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub path: PathBuf,
    pub debounce_time: Duration,
    // TODO commit_msg:
}

impl RepositoryConfig {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            debounce_time: Duration::from_secs(360),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    pub repositories: Vec<RepositoryConfig>,
}

pub fn add_repo(path: &Vec<PathBuf>) -> Result<(), anyhow::Error> {
    let mut cfg: Configuration = confy::load("git_afk", None)?;
    for repo_path in path {
        let absolute_path = std::path::absolute(repo_path)
            .unwrap()
            .canonicalize()
            .unwrap();
        // TODO check if the folder is a git repository
        // TODO check if the folder isn't already added

        let repo_cfg = RepositoryConfig::new(&absolute_path);
        cfg.repositories.push(repo_cfg);

        let dir_name = absolute_path.file_name();
        info!("{dir_name:?} has been added to afk watching");
        println!("{dir_name:?} has been added to afk watching");
    }
    confy::store("git_afk", None, cfg)?;
    Ok(())
}
