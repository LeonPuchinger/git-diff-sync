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

pub fn fetch_repo(repo: &Repository) -> Result<(), error::Error> {
    let remotes = repo.remotes()?;
    if remotes.is_empty() {
        Err(internal!("repo has no remotes. cannot pull/fetch"))?;
    }
    let mut remote = repo.find_remote(remotes.get(0).unwrap())?;
    let branch = current_branch(repo)?;
    remote.fetch(&[branch], None, None)?;
    Ok(())
}
