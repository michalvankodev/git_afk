use chrono::Local;
use core::str;
use log::{debug, info};
use std::path::{Path, PathBuf};
use tokio::process::Command;

pub async fn commit_and_push(path: PathBuf) -> Result<(), anyhow::Error> {
    debug!("Checking for commiting changes to {:?}", &path);

    // Check the status of the repository
    if is_repo_in_rebase_or_merge(path.clone()) {
        info!("Repository is in state which is not suitable for automatic commit!");
        // return Err(anyhow!(
        //     "Repository is not in state suitable for automatic commit"
        // ));
        return Ok(());
    }

    if !has_uncommitted_changes(path.clone()).await? {
        info!("No commitable changes in {:?}", path.clone());
        return Ok(());
    }

    git_commit(path.clone()).await?;

    Ok(())
}

fn is_repo_in_rebase_or_merge(path: PathBuf) -> bool {
    let git_dir = Path::new(&path).join(".git");

    // Check for rebase directory
    if git_dir.join("rebase-apply").exists() || git_dir.join("rebase-merge").exists() {
        return true;
    }

    // Check for merge state (MERGE_HEAD indicates a merge conflict)
    if git_dir.join("MERGE_HEAD").exists() {
        return true;
    }

    false
}

async fn has_uncommitted_changes(path: PathBuf) -> Result<bool, anyhow::Error> {
    let status = Command::new("git")
        .current_dir(&path)
        .arg("status")
        .arg("--porcelain")
        .output()
        .await?;
    let result = str::from_utf8(&status.stdout)?;
    debug!("Repository {:?} status: {:?}", &path, result);

    Ok(!result.is_empty())
}

async fn git_commit(path: PathBuf) -> Result<(), anyhow::Error> {
    let date = Local::now();
    let formatted_date = date.to_rfc2822();
    let commit_msg = format!("Autocommitted by git_afk @ {}", formatted_date);
    info!("Committing changes to {:?}: {}", &path, &commit_msg);

    let _add = Command::new("git")
        .current_dir(&path)
        .arg("add")
        .arg(".")
        .output()
        .await?;

    let _commit = Command::new("git")
        .current_dir(&path)
        .arg("commit")
        .arg("-m")
        .arg(&commit_msg)
        .output()
        .await?;

    Ok(())
}
