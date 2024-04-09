use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, Confirm};
use is_executable::IsExecutable;
use itertools::Itertools;
use walkdir::WalkDir;
use which::which_all;

use crate::config::{Config, JavaHome};

/// Asks the user for every path if they want to add it to jvenv and adds them to the config if desired
fn ask<I>(java_homes: I, config: &mut Config)
where
    I: Iterator<Item = PathBuf>,
{
    for java_home in java_homes {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "JEnv found a java executable in: {}. Do you want to add it?",
                java_home.to_string_lossy()
            ))
            .interact()
            .unwrap()
        {
            config.java_homes.push(JavaHome::new(java_home));
            if let Err(e) = config.save() {
                println!("- There was an error saving the config file: {}", e);
            } else {
                println!("- Saved")
            }
        }
    }
}

/// Filters an iterator of paths:
/// - only returns files name java
/// - removes symlinks / shortcuts
/// - only executable files
/// - converts them to absolute paths
/// - removes any blacklisted JavaHomes
fn filter_paths<'a, I>(blacklist: &'a Vec<JavaHome>, paths: I) -> impl Iterator<Item = PathBuf> + 'a
where
    I: Iterator<Item = PathBuf> + 'a,
{
    paths
        .filter(|entry| entry.file_name().is_some_and(|name| name == "java"))
        .filter(|path| !path.is_symlink())
        .filter(|path| path.is_executable())
        .filter_map(|path| path.canonicalize().ok())
        .filter(move |path| !blacklist.iter().any(|java_home| &java_home.path == path))
        .unique()
}

pub fn execute() -> () {
    let mut config = Config::load();

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want JEnv to search for a java executable in your path?")
        .interact()
        .unwrap()
    {
        ask(
            filter_paths(&config.java_homes.clone(), which_all("java").unwrap()),
            &mut config,
        );
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want JEnv to search for a java executable in your entire filesystem?")
        .interact()
        .unwrap()
    {
        println!("Once all java homes have been found just aboard execution with ctrl+c to avoid going through all your files.");
        ask(
            filter_paths(
                &config.java_homes.clone(),
                WalkDir::new("/")
                    .follow_links(true)
                    .into_iter()
                    // Can be skipped for performance. Only contains hardware information
                    .filter_entry(|entry| !entry.path().starts_with("/proc"))
                    .filter_map(|e| e.ok())
                    .map(|entry| entry.into_path()),
            ),
            &mut config,
        );
    }
}
