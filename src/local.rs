use crate::error;
use git2::{ApplyLocation, Diff, DiffOptions, Repository};
use std::{fs::File, io::prelude::*, path::Path};

pub fn init_local(path: &Path) -> Result<Repository, error::Error> {
    let repo = Repository::open(path)?;
    Ok(repo)
}

pub fn generate_diff(repo: &Repository) -> Result<(Diff, String), error::Error> {
    let mut opt = DiffOptions::new();
    opt.show_untracked_content(true);
    opt.recurse_untracked_dirs(true);
    let head = repo.head()?;
    let tree = head.peel_to_tree()?;
    let diff = repo.diff_tree_to_workdir(Some(&tree), Some(&mut opt))?;
    let branch = head.shorthand().unwrap();
    Ok((diff, branch.to_owned()))
}

pub fn apply_diff(repo: &Repository) -> Result<(), error::Error> {
    let mut file = File::open("diff.txt")?;
    let metadata = file.metadata()?;
    let mut buf = vec![0; metadata.len() as usize];
    file.read(&mut buf)?;
    let diff = Diff::from_buffer(&buf)?;
    repo.apply(&diff, ApplyLocation::WorkDir, None)?;
    Ok(())
}
