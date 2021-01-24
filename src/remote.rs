use crate::error;
use dirs::home_dir;
use git2::Repository;
use std::{fs, path::PathBuf};

fn config_dir() -> Result<PathBuf, error::Error> {
    let home = match home_dir() {
        Some(path) => path,
        None => Err(internal!("could not find home directory"))?,
    };
    let config = home.join(".git-diff-sync");
    if !config.exists() {
        fs::create_dir(&config)?;
    }
    Ok(config)
}

pub fn init_remote_repo() -> Result<Repository, error::Error> {
    let config = config_dir()?;
    let repo = Repository::open(config.join("git-sync"))?;
    Ok(repo)
}
