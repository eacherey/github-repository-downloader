mod cli;

use clap::Parser;
use cli::{Cli, Command};
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::User {
            name,
            token,
            output,
        } => {
            println!("user; {}; {:?}; {:?}", name, token, output);
        }

        Command::Org {
            name,
            token,
            output,
        } => {
            println!("org; {}; {:?}; {:?}", name, token, output);
        }

        Command::Token { token } => {
            println!("token; {}", token);
        }
    }
}
