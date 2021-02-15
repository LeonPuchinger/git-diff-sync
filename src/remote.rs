use crate::error;
use crate::config;
use git2::{Repository, Diff, DiffDelta, DiffFormat, DiffHunk, DiffLine};
use std::{io::prelude::*, path::Path};
use std::fs::{File, create_dir_all};
use chrono::{Local};

pub fn init_remote() -> Result<Repository, error::Error> {
    let repo = Repository::open(config::remote_path()?)?;
    Ok(repo)
}

fn commit_diff(repo: &Repository, path: &Path, local_name: &str) -> Result<(), error::Error> {
    let mut index = repo.index()?;
    let relative_path = path.strip_prefix(repo.path().parent().unwrap())?;
    index.add_path(relative_path)?;
    let sig = repo.signature()?;
    let now = Local::now().format("%Y-%m-%d-%H:%M:%S:%3f").to_string();
    let mut msg = String::from(local_name);
    msg.truncate(25);
    msg.push('_');
    msg.push_str(&now);
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    match repo.head() {
        Ok(head) => {
            let parent = head.peel_to_commit()?;
            repo.commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[&parent])?;
        },
        Err(_) => {
            repo.commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[])?;
        }
    };
    Ok(())
}

pub fn save_diff(repo: &Repository, diff: &Diff, local_name: &str, branch: &str) -> Result<(), error::Error> {
    let mut diff_path = config::remote_path()?.join(local_name);
    create_dir_all(diff_path.as_path())?;
    diff_path = diff_path.join(branch);
    diff_path.set_extension("diff");
    let mut file = File::create(&diff_path)?;

    let read_diff_line = |_delta: DiffDelta, _hunk: Option<DiffHunk>, line: DiffLine| -> bool {
        let origin = line.origin();
        if origin == '+' || origin == '-' || origin == ' ' {
            if let Err(_) = file.write(&[origin as u8]) {
                return false;
            };
        }
        if let Err(_) = file.write(line.content()) {
            return false;
        };
        true
    };

    diff.print(DiffFormat::Patch, read_diff_line)?;
    commit_diff(repo, &diff_path, local_name)?;
    Ok(())
}

fn active_branch(repo: &Repository) -> Result<String, error::Error> {
    let head = repo.head()?;
    let name = head.shorthand().unwrap();
    Ok(name.to_owned())
}

pub fn refresh_remote(repo: &Repository) -> Result<(), error::Error> {
    let remotes = repo.remotes()?;
    if remotes.is_empty() {
        Err(internal!("repo has no remotes. cannot pull/fetch"))?;
    }
    let mut remote = repo.find_remote(remotes.get(0).unwrap())?;
    let branch = active_branch(repo)?;
    remote.fetch(&[branch], None, None)?;
    Ok(())
}
