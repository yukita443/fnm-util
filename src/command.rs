use crate::package::{install_packages, packages_of};
use crate::version::{format_node_version, install_node, is_available, is_installed};
use crate::AppError;
use colored::Colorize;
use duct::cmd;
use std::io::{self, Write};

pub fn install(version: &str, packages_version: &str) -> Result<(), AppError> {
    if is_installed(version, false) {
        return Err(AppError::AlreadyInstalled {
            version: version.to_string(),
        });
    }

    if !is_available(version) {
        return Err(AppError::CannotFindRemoteVersion {
            version: version.to_string(),
        });
    }

    if is_installed(packages_version, true) {
        let packages = packages_of(packages_version);
        let packages: Vec<&str> = packages.iter().map(String::as_str).collect();

        install_node(version, false);

        println!();

        install_packages(version, packages_version, &packages);
    } else {
        install_node(version, false);

        println!();

        if packages_version == version {
            return Err(AppError::CannotReinstallPackages {
                version: format_node_version(packages_version),
            });
        } else {
            return Err(AppError::CannotFindVersion {
                version: packages_version.to_string(),
            });
        }
    }

    Ok(())
}

pub fn update(packages_version: Option<&str>) -> Result<(), AppError> {
    let output = cmd!("fnm", "list-remote")
        .read()
        .expect("Failed to run `fnm list-remote`");
    let latest = output.lines().last().unwrap();

    if is_installed(latest, false) {
        println!("Node is up-to-date.");
        return Ok(());
    }

    println!("The latest version {} is not installed.", latest.cyan());
    print!("Do you want to install and set it as default? [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read user input");

    if input.trim().to_lowercase() != "y" {
        return Ok(());
    }

    println!();

    match packages_version {
        Some(packages_version) if is_installed(packages_version, true) => {
            let packages = packages_of(packages_version);
            let packages: Vec<&str> = packages.iter().map(String::as_str).collect();

            install_node(latest, true);

            println!();

            install_packages(latest, packages_version, &packages);
        }
        Some(packages_version) => {
            install_node(latest, true);

            println!();

            if packages_version == latest {
                return Err(AppError::CannotReinstallPackages {
                    version: format_node_version(packages_version),
                });
            } else {
                return Err(AppError::CannotFindVersion {
                    version: packages_version.to_string(),
                });
            }
        }
        None => install_node(latest, true),
    }

    Ok(())
}
