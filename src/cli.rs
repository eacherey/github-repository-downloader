use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "github-dl",
    alias = "gh-dl",
    about = "CLI utility for mass cloning all repositories of a GitHub user or organization"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "user", about = "Cloning all repositories of a GitHub user")]
    User {
        #[arg(help = "GitHub username to clone repositories from")]
        name: String,

        #[arg(short, long, help = "GitHub token for authenticated API requests")]
        token: Option<String>,

        #[arg(short, long, help = "Output directory for cloned repositories")]
        output: Option<PathBuf>,
    },

    #[command(
        name = "org",
        about = "Cloning all repositories of a GitHub organization"
    )]
    Org {
        #[arg(help = "GitHub organization to clone repositories from")]
        name: String,

        #[arg(short, long, help = "GitHub token for authenticated API requests")]
        token: Option<String>,

        #[arg(short, long, help = "Output directory for cloned repositories")]
        output: Option<PathBuf>,
    },

    #[command(
        name = "token",
        about = "Save GitHub personal access token to local configuration"
    )]
    Token {
        #[arg(help = "GitHub personal access token with repo scope")]
        token: String,
    },
}
