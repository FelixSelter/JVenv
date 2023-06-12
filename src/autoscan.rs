use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, Confirm};
use is_executable::IsExecutable;
use itertools::Itertools;
use walkdir::WalkDir;
use which::which_all;

use crate::config::{Config, JavaHome};

fn ask<I>(java_homes: I) -> Vec<JavaHome>
where
    I: Iterator<Item = PathBuf>,
{
    let mut r = Vec::new();

    for java_home in java_homes {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "JEnv found a java executable in: {}. Do you want to add it?",
                java_home.to_string_lossy()
            ))
            .interact()
            .unwrap()
        {
            r.push(JavaHome::new(java_home));
        }
    }

    r
}

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
        for java_home in ask(filter_paths(&config.java_homes, which_all("java").unwrap())) {
            config.java_homes.push(java_home);
        }
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want JEnv to search for a java executable in your entire filesystem?")
        .interact()
        .unwrap()
    {
        for java_home in ask(filter_paths(
            &config.java_homes,
            WalkDir::new("/")
                .follow_links(true)
                .into_iter()
                .filter_entry(|entry| !entry.path().starts_with("/proc"))
                .filter_map(|e| e.ok())
                .map(|entry| entry.into_path()),
        )) {
            config.java_homes.push(java_home);
        }
    }
}
