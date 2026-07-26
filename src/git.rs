use anyhow::{Result, anyhow, bail};
use std::path::PathBuf;
use std::process::Command;

pub fn clone_url(url: &str, path: PathBuf) -> Result<()> {
    let mut cmd = Command::new("git");
    let path_str = path
        .to_str()
        .ok_or_else(|| anyhow!("Invalid destination path"))?;

    cmd.args(["clone", url, path_str]);

    let status = cmd.status()?;

    if !status.success() {
        bail!("git clone failed with exit code: {:?}", status.code());
    }

    Ok(())
}
