mod model;
mod score;
mod storage;

use std::{env, io, path::PathBuf, process};

use serde::Serialize;

fn main() {
    if let Err(error) = run() {
        eprintln!("rolign: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let (command, workspace) = parse_args()?;
    match command.as_str() {
        "save" => {
            let request = serde_json::from_reader(io::stdin())
                .map_err(|error| format!("invalid JSON input: {error}"))?;
            let root = storage::workspace_root(&workspace)?;
            let resume = storage::read_resume(&root)?;
            let draft = model::validate_request(request, resume)?;
            write_json(&storage::save(&root, &draft)?)
        }
        "list" => write_json(&storage::list(&workspace)?),
        _ => unreachable!("arguments are validated by parse_args"),
    }
}

fn parse_args() -> Result<(String, PathBuf), String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    if command != "save" && command != "list" {
        return Err(usage());
    }

    let mut workspace =
        env::current_dir().map_err(|error| format!("cannot read current directory: {error}"))?;
    if let Some(flag) = args.next() {
        if flag != "--workspace" {
            return Err(usage());
        }
        workspace = args.next().map(PathBuf::from).ok_or_else(usage)?;
    }
    if args.next().is_some() {
        return Err(usage());
    }
    Ok((command, workspace))
}

fn usage() -> String {
    "usage: rolign <save|list> [--workspace PATH]".to_owned()
}

fn write_json(value: &impl Serialize) -> Result<(), String> {
    serde_json::to_writer(io::stdout(), value)
        .map_err(|error| format!("cannot write JSON output: {error}"))?;
    println!();
    Ok(())
}
