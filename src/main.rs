use std::{env, process};

#[macro_use]
mod error;
mod local;
mod remote;

fn run() -> Result<(), error::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        Err(internal!("usage: git-diff-sync <-a or -g> <path-to-repo>"))?;
    }
    let path = &args[2];
    let local = local::init_local_repo(path)?;
    let remote = remote::init_remote_repo()?;
    if args[1] == "-g" {
        local::generate_diff(&local)?;
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
