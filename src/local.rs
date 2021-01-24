use crate::error;
use git2::{
    ApplyLocation, Diff, DiffDelta, DiffFormat, DiffHunk, DiffLine, DiffOptions, Repository,
};
use std::{fs::File, io::prelude::*};

pub fn init_local_repo(path: &str) -> Result<Repository, error::Error> {
    let repo = Repository::open(path)?;
    Ok(repo)
}

pub fn generate_diff(repo: &Repository) -> Result<(), error::Error> {
    let mut opt = DiffOptions::new();
    opt.show_untracked_content(true);
    opt.recurse_untracked_dirs(true);
    let head = repo.head()?;
    let tree = head.peel_to_tree()?;
    let diff = repo.diff_tree_to_workdir(Some(&tree), Some(&mut opt))?;
    let mut file = File::create("diff.txt")?;

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
    Ok(())
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
