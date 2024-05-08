use crate::{
    version::{format_node_version, use_node},
    AppError,
};
use duct::cmd;
use log::debug;
use std::{path::Path, process::Command};

pub fn install_packages(
    version: &str,
    packages_version: &str,
    packages: &[&str],
) -> Result<(), AppError> {
    let current = cmd!("fnm", "current").read()?;
    debug!("Current Node version: {current}");

    use_node(version)?;

    println!(
        "Reinstalling global packages from {}",
        format_node_version(packages_version)?
    );

    Command::new("npm")
        .args(["install", "--global"])
        .args(packages)
        .output()?;

    use_node(&current)?;

    Ok(())
}

pub fn packages_of(version: &str) -> Result<Vec<String>, AppError> {
    let current = cmd!("fnm", "current").read()?;
    debug!("Current Node version: {current}");

    use_node(version)?;

    let output = cmd!("npm", "list", "--global", "--parseable").read()?;
    let list = output.lines().skip(1);

    use_node(&current)?;

    Ok(list
        .filter_map(|s| Path::new(s).file_name())
        .filter_map(|s| s.to_os_string().into_string().ok())
        .collect())
}
