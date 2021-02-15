use crate::error;
use crate::config;
use git2::{Repository, Diff, DiffDelta, DiffFormat, DiffHunk, DiffLine};
use std::{fs::File, fs::create_dir_all, io::prelude::*};

pub fn init_remote() -> Result<Repository, error::Error> {
    let repo = Repository::open(config::remote_path()?)?;
    Ok(repo)
}

pub fn save_diff(_repo: &Repository, diff: &Diff, local_name: &str, branch: &str) -> Result<(), error::Error> {
    let mut diff_file = config::remote_path()?.join(local_name);
    create_dir_all(diff_file.as_path())?;
    diff_file = diff_file.join(branch);
    diff_file.set_extension("diff");
    let mut file = File::create(diff_file)?;

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
    Ok(config)
}

pub fn init_remote_repo() -> Result<Repository, error::Error> {
    let config = config_dir()?;
    let repo = Repository::open(config.join("git-sync"))?;
    Ok(repo)
}
