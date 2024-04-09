use std::path::PathBuf;

use clap::{builder::Str, Parser, Subcommand};

mod commands;
mod config;

#[derive(Debug, Parser)]
#[clap(
    name = "JVenv",
    author = "Felix Selter",
    version,
    about = "description"
)]
struct Arguments {
    #[clap(subcommand)]
    command: Action,
}

#[derive(Subcommand, Debug)]

enum Action {
    AutoScan,
    Register { name: String, path: PathBuf },
    Unregister { name: String },
    List,
    JavaHome,
    Install { name: String },
    Uninstall { name: String },
    Integrate,
    UninstallJVenv,
    Global { name: String },
    Use { name: String },
    Init { name: String },
    Restore,
}

fn main() {
    match Arguments::parse().command {
        Action::AutoScan => commands::autoscan::execute(),
        Action::Register { name, path } => commands::register::execute(name, path),
        Action::Unregister { name } => commands::unregister::execute(name),
        Action::List => commands::list::execute(),
        Action::JavaHome => commands::javahome::execute(),
        Action::Install { name } => commands::install::execute(name),
        Action::Uninstall { name } => commands::uninstall::execute(name),
        Action::Integrate => commands::integrate::execute(),
        Action::UninstallJVenv => commands::uninstalljvenv::execute(),
        Action::Global { name } => commands::global::execute(name),
        Action::Use { name } => commands::r#use::execute(name),
        Action::Init { name } => commands::init::execute(name),
        Action::Restore => commands::restore::execute(),
    }
}
