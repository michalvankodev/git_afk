use std::time::Duration;

use log::{debug, info};
use tokio::time::sleep;

pub async fn start_watcher() {
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
