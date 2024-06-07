use anyhow::Context;
use colored::Colorize;
use duct::cmd;
use log::debug;

pub fn install_node(version: &str, set_as_default: bool) -> anyhow::Result<()> {
    println!("Installing Node {}", version.cyan());

    cmd!("fnm", "install", version)
        .read()
        .with_context(|| format!("Failed to run `fnm install {version}`"))?;

    if set_as_default {
        println!("Setting Node {} as default.", version.cyan());

        cmd!("fnm", "default", version)
            .read()
            .with_context(|| format!("Failed to run `fnm default {version}`"))?;

        use_node(version)?;
    }

    Ok(())
}

pub fn use_node(version: &str) -> anyhow::Result<()> {
    debug!("Using Node {version}");

    cmd!("fnm", "use", version)
        .read()
        .with_context(|| format!("Failed to run `fnm use {version}`"))?;

    Ok(())
}

pub fn local_node_exists(version: &str, alias: bool) -> anyhow::Result<bool> {
    let output = cmd!("fnm", "list")
        .read()
        .context("Failed to run `fnm list`")?;
    let mut list = output.lines();

    if alias {
        Ok(list.any(|line| line.split_whitespace().skip(1).any(|s| s == version)))
    } else {
        Ok(list.any(|line| line.split_whitespace().nth(1).is_some_and(|s| s == version)))
    }
}

pub fn remote_node_exists(version: &str) -> anyhow::Result<bool> {
    let output = cmd!("fnm", "list-remote")
        .read()
        .context("Failed to run `fnm list-remote`")?;
    let mut list_remote = output.lines();

    Ok(list_remote.any(|line| line.split_whitespace().nth(0).is_some_and(|s| s == version)))
}

pub fn format_node_version(version: &str) -> anyhow::Result<String> {
    let output = cmd!("fnm", "list")
        .read()
        .context("Failed to run `fnm list`")?;
    let mut list = output.lines();

    if list.any(|line| line.split_whitespace().nth(2).is_some_and(|s| s == version)) {
        Ok(format!("Node for alias {}", version.cyan()))
    } else {
        Ok(format!("Node {}", version.cyan()))
    }
}
