use anyhow::{Context, Result};
use dirs::config_dir;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = config_dir().context("Failed to get config dir")?;

    Ok(config_dir.join("github-dl"))
}
pub fn save_token(token: &str) -> Result<()> {
    let config_dir = get_config_dir()?;

    create_dir_all(&config_dir).context("Failed to create config dir")?;

    let token_path = config_dir.join("github-token");

    write(&token_path, token).context("Failed to save token")?;

    Ok(())
}
pub fn load_token() -> Option<String> {
    let config_dir = get_config_dir().ok()?;
    let token_path = config_dir.join("github-token");

    if !token_path.exists() {
        return None;
    }

    read_to_string(&token_path)
        .ok()
        .map(|content| content.trim().to_string())
}
