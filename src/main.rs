use git2::{
    ApplyLocation, Diff, DiffDelta, DiffFormat, DiffHunk, DiffLine, DiffOptions, Repository,
};
use std::{env, fs::File, io::prelude::*, process};

mod error;

fn generate_diff(path: &str) -> Result<(), error::Error> {
    let mut opt = DiffOptions::new();
    opt.show_untracked_content(true);
    opt.recurse_untracked_dirs(true);

    println!("opening repo at {}", path);
    let repo = Repository::init(path)?;
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
        return true;
    };

    diff.print(DiffFormat::Patch, read_diff_line)?;

    Ok(())
}

fn apply_diff(path: &str) -> Result<(), error::Error> {
    let mut file = File::open("diff.txt")?;
    let metadata = file.metadata()?;
    println!("{}", metadata.len());
    let mut buf = vec![0; metadata.len() as usize];
    file.read(&mut buf)?;
    let repo = Repository::init(path)?; //call this before from_buffer, so git_libgit2_init gets called
    let diff = Diff::from_buffer(&buf)?;
    repo.apply(&diff, ApplyLocation::WorkDir, None)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("USAGE: git-diff-sync <path-to-repo>");
        process::exit(1);
    }
    let path = &args[1];
    if let Err(e) = generate_diff(path) {
        println!("{}", e);
        process::exit(1);
    }
}
