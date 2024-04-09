use std::{io::Read, path::PathBuf, process::Command};

use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use is_executable::IsExecutable;
use itertools::Itertools;
use walkdir::WalkDir;
use which::which_all;

use crate::{
    config::{Config, JavaHome},
    util::parse_java_version,
};

/// Asks the user for every path if they want to add it to jvenv and adds them to the config if desired
fn add_to_config_on_confirm<I>(java_executables: I, config: &mut Config)
where
    I: Iterator<Item = PathBuf>,
{
    for java_executable in java_executables {
        let version = parse_java_version(&java_executable);

        println!("");
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "JEnv found a java executable:\n- Path: {}\n- Version: {}\nDo you want to add it?",
                java_executable.to_string_lossy(),
                match &version {
                    Ok(v) => v.to_string(),
                    Err(e) => format!("CANNOT RETRIEVE VERSION INFORMATION. Error: {:?}", e),
                }
            ))
            .report(false)
            .interact()
            .unwrap()
        {
            let name = Input::<String>::new()
                .with_prompt("Name this java installation: ")
                .with_initial_text(match &version {
                    Ok(v) => format!("{}{}", v.distribution, v.major),
                    Err(_) => String::new(),
                })
                .interact_text()
                .unwrap();

            config.java_homes.push(JavaHome::new(
                java_executable
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_path_buf(),
                name,
            ));
            if let Err(e) = config.save() {
                println!("There was an error saving the config file: {}", e);
            } else {
                println!("Saved")
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
        .filter(move |path| {
            !blacklist
                .iter()
                .any(|java_home| &java_home.path == path.parent().unwrap().parent().unwrap())
        })
        .unique()
}

pub fn execute() -> () {
    let mut config = Config::load();

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want JEnv to search for a java executable in your path?")
        .interact()
        .unwrap()
    {
        add_to_config_on_confirm(
            filter_paths(&config.java_homes.clone(), which_all("java").unwrap()),
            &mut config,
        );
    }

    println!("");
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want JEnv to search for a java executable in your entire filesystem?")
        .interact()
        .unwrap()
    {
        println!("Once all java homes have been found just aboard execution with ctrl+c to avoid going through all your files.");
        add_to_config_on_confirm(
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
