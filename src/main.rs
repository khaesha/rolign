mod model;
mod score;
mod storage;

use std::{env, io, path::PathBuf, process};

use serde::Serialize;

enum Command {
    Save(PathBuf),
    List(PathBuf),
    ValidateUrl(String),
}

fn main() {
    if let Err(error) = run() {
        eprintln!("rolign: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    match parse_args()? {
        Command::Save(workspace) => {
            let request = serde_json::from_reader(io::stdin())
                .map_err(|error| format!("invalid JSON input: {error}"))?;
            let root = storage::workspace_root(&workspace)?;
            let resume = storage::read_resume(&root)?;
            let draft = model::validate_request(request, resume)?;
            write_json(&storage::save(&root, &draft)?)
        }
        Command::List(workspace) => write_json(&storage::list(&workspace)?),
        Command::ValidateUrl(url) => write_json(&ValidatedUrl {
            url: model::validate_url(url)?,
        }),
    }
}

#[derive(Serialize)]
struct ValidatedUrl {
    url: String,
}

fn parse_args() -> Result<Command, String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    if command == "validate-url" {
        let url = args.next().ok_or_else(usage)?;
        return if args.next().is_none() {
            Ok(Command::ValidateUrl(url))
        } else {
            Err(usage())
        };
    }
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
    Ok(if command == "save" {
        Command::Save(workspace)
    } else {
        Command::List(workspace)
    })
}

fn usage() -> String {
    "usage: rolign <save|list> [--workspace PATH] | rolign validate-url <URL>".to_owned()
}

fn write_json(value: &impl Serialize) -> Result<(), String> {
    serde_json::to_writer(io::stdout(), value)
        .map_err(|error| format!("cannot write JSON output: {error}"))?;
    println!();
    Ok(())
}
