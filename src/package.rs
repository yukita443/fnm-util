use crate::version::{format_node_version, use_node};
use anyhow::Context;
use duct::cmd;
use log::debug;
use std::{path::Path, process::Command};

pub fn install_packages(
    version: &str,
    packages_version: &str,
    packages: &[&str],
) -> anyhow::Result<()> {
    let current = cmd!("fnm", "current")
        .read()
        .context("Failed to run `fnm current`")?;
    debug!("Current Node version: {current}");

    use_node(version)?;

    println!(
        "Reinstalling global packages from {}",
        format_node_version(packages_version)?
    );

    Command::new("npm")
        .args(["install", "--global"])
        .args(packages)
        .output()
        .with_context(|| {
            format!(
                "Failed to run `npm install --global {}`",
                packages.join(" ")
            )
        })?;

    use_node(&current)?;

    Ok(())
}

pub fn packages_of(version: &str) -> anyhow::Result<Vec<String>> {
    let current = cmd!("fnm", "current")
        .read()
        .context("Failed to run `fnm current`")?;
    debug!("Current Node version: {current}");

    use_node(version)?;

    let output = cmd!("npm", "list", "--global", "--parseable")
        .read()
        .context("Failed to run `npm list --global --parseable`")?;
    let list = output.lines().skip(1);

    use_node(&current)?;

    list.map(|path| {
        Path::new(path)
            .file_name()
            .and_then(|s| s.to_os_string().into_string().ok())
    })
    .collect::<Option<_>>()
    .context("Failed to get list of installed packages")
}
