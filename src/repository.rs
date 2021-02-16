use crate::error;
use git2::{Repository};
use std::path::Path;

pub fn open_repo(path: &Path) -> Result<Repository, error::Error> {
    let repo = Repository::open(path)?;
    Ok(repo)
}

pub fn current_branch(repo: &Repository) -> Result<String, error::Error> {
    let head = repo.head()?;
    let name = head.shorthand().unwrap();
    Ok(name.to_owned())
}
