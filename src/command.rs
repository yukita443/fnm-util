use crate::package::{install_packages, packages_of};
use crate::version::{format_node_version, install_node, local_node_exists, remote_node_exists};
use crate::AppError;
use colored::Colorize;
use duct::cmd;
use std::io::{self, Write};

pub fn install(version: &str, packages_version: &str) -> Result<(), AppError> {
    if local_node_exists(version, false)? {
        return Err(AppError::AlreadyInstalled {
            version: version.to_string(),
        });
    }

    if !remote_node_exists(version)? {
        return Err(AppError::CannotFindRemoteVersion {
            version: version.to_string(),
        });
    }

    if local_node_exists(packages_version, true)? {
        let packages = packages_of(packages_version)?;
        let packages: Vec<&str> = packages.iter().map(String::as_str).collect();

        install_node(version, false)?;

        println!();

        install_packages(version, packages_version, &packages)?;
    } else {
        install_node(version, false)?;

        println!();

        return if packages_version == version {
            Err(AppError::CannotReinstallPackages {
                version: format_node_version(packages_version)?,
            })
        } else {
            Err(AppError::CannotFindVersion {
                version: packages_version.to_string(),
            })
        };
    }

    Ok(())
}

pub fn update(packages_version: Option<&str>) -> Result<(), AppError> {
    let output = cmd!("fnm", "list-remote").read()?;
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
            let packages: Vec<&str> = packages.iter().map(String::as_str).collect();

            install_node(latest, true)?;

            println!();

            install_packages(latest, packages_version, &packages)?;
        }
        Some(packages_version) => {
            install_node(latest, true)?;

            println!();

            return if packages_version == latest {
                Err(AppError::CannotReinstallPackages {
                    version: format_node_version(packages_version)?,
                })
            } else {
                Err(AppError::CannotFindVersion {
                    version: packages_version.to_string(),
                })
            };
        }
        None => install_node(latest, true)?,
    }

    Ok(())
}
