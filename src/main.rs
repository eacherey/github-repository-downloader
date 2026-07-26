mod cli;
mod config;
mod git;
mod github;

use crate::{
    cli::{Cli, Command, TargetType},
    config::{load_token, save_token},
    git::clone_url,
    github::fetch_repos,
};
use anyhow::Result;
use clap::Parser;
use std::env::current_dir;
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::User {
            name,
            token,
            output,
        } => {
            handle_download(TargetType::User, &name, token, output)?;
        }
        Command::Org {
            name,
            token,
            output,
        } => {
            handle_download(TargetType::Org, &name, token, output)?;
        }

        Command::Token { token } => {
            save_token(&token)?;
        }
    }

    Ok(())
}

fn handle_download(
    target_type: TargetType,
    name: &str,
    token: Option<String>,
    output: Option<PathBuf>,
) -> Result<()> {
    let gh_token = token.or_else(load_token);

    if gh_token.is_some() {
        println!("Use authorization");
    } else {
        println!("No authorization found, limits apply (60 req/h)");
    }

    let all_repos = fetch_repos(target_type, name, &gh_token);

    let mut cloned = 0;
    let mut skipped = 0;
    let mut errors = 0;

    let path_output = match output {
        Some(path) => path,
        None => current_dir()?,
    };

    if let Ok(repos) = all_repos {
        for repo in repos {
            let path = path_output.join(&repo.name);

            if path.exists() {
                println!("{} already exists", repo.name);
                skipped += 1;
                continue;
            }

            let repo_url = if let Some(token) = &gh_token {
                repo.clone_url
                    .replace("https://", &format!("https://{}@", token))
            } else {
                repo.clone_url.clone()
            };

            println!("Cloning {}...", repo.name);

            match clone_url(&repo_url, path) {
                Ok(()) => {
                    println!("{} cloned", repo.name);
                    cloned += 1;
                }
                Err(e) => {
                    eprintln!("Error cloning {}: {}", repo.name, e);
                    errors += 1;
                }
            }
        }
    }

    println!("{} repos cloned", cloned);
    if skipped > 0 {
        println!("{} skipped", skipped);
    }
    if errors > 0 {
        println!("{} error", errors);
    }

    Ok(())
}
