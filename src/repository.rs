use crate::error;
use dirs::home_dir;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
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
    let mut fetch_opt = FetchOptions::default();
    let mut callbacks = RemoteCallbacks::default();
    callbacks.credentials(|_url, usr_url: Option<&str>, _allowed_types| {
        let priv_key_path = home_dir().unwrap().join(".ssh/id_rsa");
        let mut pub_key_path = priv_key_path.to_owned();
        pub_key_path.set_extension("pub");
        let cred = Cred::ssh_key(usr_url.unwrap(), Some(&pub_key_path), &priv_key_path, None)?;
        Ok(cred)
    });
    fetch_opt.remote_callbacks(callbacks);
    remote.fetch(&[branch], Some(&mut fetch_opt), None)?;
    Ok(())
}
