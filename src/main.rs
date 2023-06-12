use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod autoscan;
mod config;

#[derive(Debug, Parser)]
#[clap(name = "jenv", author = "Felix Selter", version, about = "description")]
struct Arguments {
    #[clap(subcommand)]
    command: Action,
}

#[derive(Subcommand, Debug)]

enum Action {
    AutoScan,
    Add { name: String, path: PathBuf },
    Remove { name: String },
}

fn main() {
    //Execute the command
    match Arguments::parse().command {
        Action::AutoScan => autoscan::execute(),
        Action::Add { name, path } => todo!(),
        Action::Remove { name } => todo!(),
    }
}
