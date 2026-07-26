mod cli;
mod config;
mod github;

use crate::{
    cli::{Cli, Command, TargetType},
    config::{load_token, save_token},
    github::fetch_repos,
};
use anyhow::Result;
use clap::Parser;
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

    Ok(())
}
