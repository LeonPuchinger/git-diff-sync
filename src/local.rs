use crate::error;
use git2::{ApplyLocation, Diff, DiffOptions, Repository};

pub fn generate_diff(repo: &Repository) -> Result<Diff, error::Error> {
    let mut opt = DiffOptions::new();
    opt.show_untracked_content(true);
    opt.recurse_untracked_dirs(true);
    opt.show_binary(true);
    let head = repo.head()?;
    let tree = head.peel_to_tree()?;
    let diff = repo.diff_tree_to_workdir(Some(&tree), Some(&mut opt))?;
    Ok(diff)
}

pub fn apply_diff(repo: &Repository, diff: &Diff) -> Result<(), error::Error> {
    repo.apply(&diff, ApplyLocation::WorkDir, None)?;
    Ok(())
}
