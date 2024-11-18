use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub path: PathBuf,
    pub debounce_time: Duration,
    pub commit_msg: String,
    // TODO commit_msg:
}

impl RepositoryConfig {
    pub fn new(path: &Path, debounce_time: u64, msg: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            debounce_time: Duration::from_secs(debounce_time),
            commit_msg: msg.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    pub repositories: Vec<RepositoryConfig>,
}

pub fn add_repo(path: &Vec<PathBuf>, debounce_time: u64, msg: String) -> Result<(), anyhow::Error> {
    let mut cfg: Configuration = confy::load("git_afk", None)?;
    for repo_path in path {
        let absolute_path = get_absolute_path(repo_path);

        // check if the folder is a git repository
        if !absolute_path.join(".git").exists() {
            return Err(anyhow!("Path {:?} is not a .git repository", absolute_path));
        }

        if cfg
            .repositories
            .iter()
            .any(|repo| repo.path == absolute_path)
        {
            return Err(anyhow!("{absolute_path:?} is already being watched"));
        }

        let repo_cfg = RepositoryConfig::new(&absolute_path, debounce_time, &msg);
        cfg.repositories.push(repo_cfg);

        let dir_name = absolute_path.file_name().unwrap();
        println!("{dir_name:?} has been added to afk watching");
    }
    confy::store("git_afk", None, cfg)?;
    Ok(())
}

pub fn remove_repo(path: &Vec<PathBuf>) -> Result<(), anyhow::Error> {
    let mut cfg: Configuration = confy::load("git_afk", None)?;

    for repo_path in path {
        let absolute_path = get_absolute_path(repo_path);

        if let Some(index) = cfg
            .repositories
            .iter()
            .position(|repo| repo.path == absolute_path)
        {
            cfg.repositories.remove(index);
            println!("{repo_path:?} has been removed from afk watching");
        } else {
            return Err(anyhow!("Path not found in the current configuration"));
        }
    }
    confy::store("git_afk", None, cfg)?;
    Ok(())
}

fn get_absolute_path(path: &PathBuf) -> PathBuf {
    std::path::absolute(path).unwrap().canonicalize().unwrap()
}
