use crate::error;
use dirs::home_dir;
use std::{fs, path::PathBuf};

pub fn config_dir() -> Result<PathBuf, error::Error> {
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

pub fn remote_path() -> Result<PathBuf, error::Error> {
    let config = config_dir()?;
    let remote = config.join("git-sync");
    Ok(remote)
}
