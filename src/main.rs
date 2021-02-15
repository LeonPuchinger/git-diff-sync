use std::{env, process, path::PathBuf};

#[macro_use]
mod error;
mod config;
mod local;
mod remote;

fn run() -> Result<(), error::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        Err(internal!("usage: git-diff-sync <-a or -g> <path-to-repo>"))?;
    }
    let path = PathBuf::from(&args[2]);
    if path.is_file() {
        Err(internal!("path is not a directory"))?;
    }
    let local_name = path.file_name().unwrap();
    let local = local::init_local(&path)?;
    let remote = remote::init_remote()?;
    if args[1] == "-g" {
        let (diff, branch) = local::generate_diff(&local)?;
        remote::save_diff(&remote, &diff, local_name.to_str().unwrap(), &branch)?;
    } else if args[1] == "-a" {
        local::apply_diff(&local)?;
    } else {
        Err(internal!("invalid argument"))?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        process::exit(1);
    }
}
