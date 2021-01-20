use git2::{DiffDelta, DiffFormat, DiffHunk, DiffLine, Repository};
use std::{env, fs::File, io::prelude::*, process};

mod error;

fn generate_diff(path: &str) -> Result<(), error::Error> {
    println!("opening repo at {}", path);
    let repo = Repository::init(path)?;
    let head = repo.head()?;
    let tree = head.peel_to_tree()?;
    let diff = repo.diff_tree_to_workdir(Some(&tree), None)?;
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
