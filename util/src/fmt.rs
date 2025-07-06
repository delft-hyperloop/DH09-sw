use std::path::PathBuf;
use std::process::Command;

use anyhow::bail;
use anyhow::Result;

pub fn run_fmt(on: &PathBuf, with_config: &PathBuf) -> Result<()> {
    let status = Command::new("rustfmt")
        .arg("--config-path")
        .arg(with_config)
        .arg("--edition=2024")
        .arg(on)
        .status()?;

    if !status.success() {
        bail!("rustfmt failed on `{}` with exit code {}", on.display(), status.code().unwrap_or(-1))
    }

    println!("formatted {} (@generated)", on.display());

    Ok(())
}
