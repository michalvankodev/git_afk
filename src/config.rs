use std::{path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub path: PathBuf,
    pub debounce_time: Duration,
    // TODO commit_msg:
}

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    pub repositories: Vec<RepositoryConfig>,
}
