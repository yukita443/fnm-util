use colored::Colorize;
use duct::cmd;

pub fn install_node(version: &str, set_as_default: bool) {
    println!("Installing Node {}", version.cyan());

    cmd!("fnm", "install", version)
        .read()
        .expect(&format!("Failed to run `fnm install {version}`"));

    if set_as_default {
        println!("Setting Node {} as default.", version.cyan());

        cmd!("fnm", "default", version)
            .read()
            .expect(&format!("Failed to run `fnm default {version}`"));

        use_node(version);
    }
}

pub fn use_node(version: &str) {
    cmd!("fnm", "use", version)
        .read()
        .expect(&format!("Failed to run `fnm use {version}`"));
}

pub fn is_installed(version: &str, alias: bool) -> bool {
    let output = cmd!("fnm", "list")
        .read()
        .expect("Failed to run `fnm list`");
    let mut list = output.lines();

    if alias {
        list.any(|line| line.split_whitespace().skip(1).any(|s| s == version))
    } else {
        list.any(|line| line.split_whitespace().nth(1).unwrap() == version)
    }
}

pub fn is_available(version: &str) -> bool {
    let output = cmd!("fnm", "list-remote")
        .read()
        .expect("Failed to run `fnm list-remote`");
    let mut list_remote = output.lines();

    list_remote.any(|s| s.split_whitespace().nth(0).unwrap() == version)
}

pub fn format_node_version(version: &str) -> String {
    let output = cmd!("fnm", "list")
        .read()
        .expect("Failed to run `fnm list`");
    let mut list = output.lines();

    if list.any(|line| line.split_whitespace().nth(2).is_some_and(|s| s == version)) {
        format!("Node for alias {}", version.cyan())
    } else {
        format!("Node {}", version.cyan())
    }
}
