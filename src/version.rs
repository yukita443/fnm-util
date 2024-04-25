use crate::AppError;
use colored::Colorize;
use duct::cmd;

pub fn install_node(version: &str, set_as_default: bool) -> Result<(), AppError> {
    println!("Installing Node {}", version.cyan());

    cmd!("fnm", "install", version).read()?;

    if set_as_default {
        println!("Setting Node {} as default.", version.cyan());
        cmd!("fnm", "default", version).read()?;
        use_node(version)?;
    }

    Ok(())
}

pub fn use_node(version: &str) -> Result<(), AppError> {
    cmd!("fnm", "use", version).read()?;
    Ok(())
}

pub fn local_node_exists(version: &str, alias: bool) -> Result<bool, AppError> {
    let output = cmd!("fnm", "list").read()?;
    let mut list = output.lines();

    if alias {
        Ok(list.any(|line| line.split_whitespace().skip(1).any(|s| s == version)))
    } else {
        Ok(list.any(|line| line.split_whitespace().nth(1).unwrap() == version))
    }
}

pub fn remote_node_exists(version: &str) -> Result<bool, AppError> {
    let output = cmd!("fnm", "list-remote").read()?;
    let mut list_remote = output.lines();

    Ok(list_remote.any(|s| s.split_whitespace().nth(0).unwrap() == version))
}

pub fn format_node_version(version: &str) -> Result<String, AppError> {
    let output = cmd!("fnm", "list").read()?;
    let mut list = output.lines();

    if list.any(|line| line.split_whitespace().nth(2).is_some_and(|s| s == version)) {
        Ok(format!("Node for alias {}", version.cyan()))
    } else {
        Ok(format!("Node {}", version.cyan()))
    }
}
