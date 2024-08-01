use crate::package::{install_packages, packages_of};
use crate::version::{format_node_version, install_node, local_node_exists, remote_node_exists};
use crate::AppError;
use anyhow::{bail, Context};
use colored::Colorize;
use duct::cmd;
use std::io::{self, Write};

pub fn install(version: &str, packages_version: &str) -> anyhow::Result<()> {
    if local_node_exists(version, false)? {
        bail!(AppError::AlreadyInstalled {
            version: version.to_string(),
        });
    }

    if !remote_node_exists(version)? {
        bail!(AppError::CannotFindRemoteVersion {
            version: version.to_string(),
        });
    }

    if local_node_exists(packages_version, true)? {
        let packages = packages_of(packages_version)?;
        let packages: Vec<_> = packages.iter().map(String::as_str).collect();

        install_node(version, false)?;

        println!();

        install_packages(version, packages_version, &packages)?;
    } else if packages_version == version {
        bail!(AppError::CannotReinstallPackages {
            version: format_node_version(packages_version)?,
        });
    } else {
        bail!(AppError::CannotFindVersion {
            version: packages_version.to_string(),
        });
    }

    Ok(())
}

pub fn update(packages_version: Option<&str>) -> anyhow::Result<()> {
    let output = cmd!("fnm", "list-remote")
        .read()
        .context("Failed to run `fnm list-remote`")?;
    let latest = output.lines().last().unwrap();

    if local_node_exists(latest, false)? {
        println!("Node is up-to-date.");
        return Ok(());
    }

    println!("The latest version {} is not installed.", latest.cyan());
    print!("Do you want to install and set it as default? [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() != "y" {
        return Ok(());
    }

    println!();

    match packages_version {
        Some(packages_version) if local_node_exists(packages_version, true)? => {
            let packages = packages_of(packages_version)?;
            let packages: Vec<_> = packages.iter().map(String::as_str).collect();

            install_node(latest, true)?;

            println!();

            install_packages(latest, packages_version, &packages)?;
        }
        Some(packages_version) if packages_version == latest => {
            bail!(AppError::CannotReinstallPackages {
                version: format_node_version(packages_version)?,
            });
        }
        Some(packages_version) => {
            bail!(AppError::CannotFindVersion {
                version: packages_version.to_string(),
            });
        }
        None => install_node(latest, true)?,
    }

    Ok(())
}
