use crate::error;
use dirs::home_dir;
use git2::build::CheckoutBuilder;
use git2::{AnnotatedCommit, Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

pub fn open_repo(path: &Path) -> Result<Repository, error::Error> {
    let repo = Repository::open(path)?;
    Ok(repo)
}

pub fn current_branch(repo: &Repository) -> Result<String, error::Error> {
    let head = match repo.head() {
        Ok(v) => v,
        Err(_) => return Ok(String::from("main")),
    };
    let name = head.shorthand().unwrap();
    Ok(name.to_owned())
}

pub fn fetch(repo: &Repository) -> Result<AnnotatedCommit, error::Error> {
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
    remote.fetch(&[&branch], Some(&mut fetch_opt), None)?;
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    Ok(fetch_commit)
}

pub fn pull(repo: &Repository) -> Result<(), error::Error> {
    let branch = current_branch(repo)?;
    let branch_refname = format!("refs/heads/{}", &branch);
    let fetch_commit = fetch(repo)?;
    let (merge_analysis, _) = repo.merge_analysis(&[&fetch_commit])?;
    if merge_analysis.is_unborn() {
        repo.reference(
            &branch_refname,
            fetch_commit.id(),
            true,
            &format!("creating branch: {} at {}", &branch, fetch_commit.id()),
        )?;
        repo.set_head(&branch_refname)?;
        let mut checkout_builder = CheckoutBuilder::default();
        checkout_builder.allow_conflicts(true);
        checkout_builder.force();
        repo.checkout_head(Some(&mut checkout_builder))?;
    } else if merge_analysis.is_fast_forward() {
        let mut reference = repo.find_reference(&branch_refname)?;
        reference.set_target(
            fetch_commit.id(),
            &format!("fast-forward: branch: {} to {}", &branch, fetch_commit.id()),
        )?;
        repo.set_head(&branch_refname)?;
        let mut checkout_builder = CheckoutBuilder::default();
        checkout_builder.force();
        repo.checkout_head(Some(&mut checkout_builder))?;
    } else if merge_analysis.is_normal() {
        Err(internal!(
            "normal merge during pull not yet supported. consider updating repo using git cli."
        ))?;
    }
    Ok(())
}
