mod command;
mod package;
mod version;

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{io, process::ExitCode};
use thiserror::Error;

#[derive(Debug, Parser)]
#[clap(version, about)]
struct App {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[clap(about = "Install a Node.js version")]
    Install {
        #[clap(required = true, help = "A version string")]
        version: String,

        #[clap(
            short = 'p',
            long = "reinstall-packages",
            value_name = "FROM",
            required = true,
            help = "When installing, reinstall packages installed in <FROM>"
        )]
        packages_version: String,
    },

    #[clap(about = "Install the latest Node.js version")]
    Update {
        #[clap(
            short = 'p',
            long = "reinstall-packages",
            value_name = "FROM",
            help = "When installing, reinstall packages installed in <FROM>"
        )]
        packages_version: Option<String>,
    },
}

fn main() -> ExitCode {
    env_logger::init();

    let result = match App::parse().subcommand {
        SubCommand::Install {
            version,
            packages_version,
        } => command::install(&version, &packages_version),
        SubCommand::Update { packages_version } => command::update(packages_version.as_deref()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error.to_string().red());
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug, Error)]
enum AppError {
    #[error("The requested version {version} is already installed.")]
    AlreadyInstalled { version: String },

    #[error("Can't find a Node version that matches `{version}` in remote.")]
    CannotFindRemoteVersion { version: String },

    #[error("Can't find an installed Node version that matches `{version}`")]
    CannotFindVersion { version: String },

    #[error("Can't reinstall global packages from {version}")]
    CannotReinstallPackages { version: String },

    #[error("IO Error: {source}")]
    IO {
        #[from]
        source: io::Error,
    },
}
