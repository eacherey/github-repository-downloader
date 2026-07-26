use crate::cli::TargetType;
use anyhow::{Result, bail};
use reqwest;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub clone_url: String,
}

pub fn fetch_repos(
    target_type: TargetType,
    name: &str,
    token: &Option<String>,
) -> Result<Vec<Repo>> {
    let url = match target_type {
        TargetType::User => {
            format!("https://api.github.com/users/{name}/repos")
        }
        TargetType::Org => {
            format!("https://api.github.com/orgs/{name}/repos")
        }
    };
    let client = reqwest::blocking::Client::new();

    let mut all_repos = Vec::new();
    let mut page = 1;

    loop {
        let mut request = client
            .get(&url)
            .header(USER_AGENT, "github-dl")
            .query(&[("per_page", "100"), ("page", &page.to_string())]);

        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send()?;

        if !response.status().is_success() {
            bail!(format!("GitHub API error: {}", response.status()));
        }

        let repos: Vec<Repo> = response.json()?;
        let count = repos.len();

        all_repos.extend(repos);

        if (count < 100) {
            break;
        }

        page += 1;
    }

    Ok(all_repos)
}
