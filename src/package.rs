use crate::version::{format_node_version, use_node};
use duct::cmd;
use std::{path::Path, process::Command};

pub fn install_packages(version: &str, packages_version: &str, packages: &[&str]) {
    let current = cmd!("fnm", "current")
        .read()
        .expect("Failed to run `fnm current`");

    use_node(version);

    println!(
        "Reinstalling global packages from {}",
        format_node_version(packages_version)
    );

    Command::new("npm")
        .args(["install", "--global"])
        .args(packages)
        .output()
        .expect(&format!(
            "Failed to run `npm install --global {}`",
            packages.join(" ")
        ));

    use_node(&current);
}

pub fn packages_of(version: &str) -> Vec<String> {
    let current = cmd!("fnm", "current")
        .read()
        .expect("Failed to run `fnm current`");

    use_node(version);

    let output = cmd!("npm", "list", "--global", "--parseable")
        .read()
        .expect("Failed to run `npm list --global --parseable`");
    let list = output.lines().skip(1);

    use_node(&current);

    list.map(|s| {
        Path::new(s)
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    })
    .collect()
}
