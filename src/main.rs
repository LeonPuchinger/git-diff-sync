use std::{env, process, path::PathBuf};

#[macro_use]
mod error;
mod config;
mod repository;
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
    
    let local = repository::open_repo(&path)?;
    let remote = remote::open_remote()?;
    let local_name = path.file_name().unwrap();
    let local_branch = repository::current_branch(&local)?;
    
    if args[1] == "-g" {
        let diff = local::generate_diff(&local)?;
        remote::save_diff(&remote, &diff, local_name.to_str().unwrap(), &local_branch)?;
    } else if args[1] == "-a" {
        let diff = remote::get_diff(&remote, local_name.to_str().unwrap(), &local_branch)?;
        local::apply_diff(&local, &diff)?;
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
