use core::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use std::{path::PathBuf, process::Command};

//unwrap only fails if regex is invalid
lazy_static! {
    static ref VERSION_MATCHER: Regex =
        Regex::new(r"^([a-zA-Z]+) .+?(\d+)\.(\d+)\.(\d+).+$").unwrap();
}

pub struct JavaVersion {
    pub distribution: String,
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl fmt::Display for JavaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}.{}.{}",
            self.distribution, self.major, self.minor, self.patch
        )
    }
}

#[derive(Debug)]
pub enum JavaVersionParsingError {
    JavaExecutionError(std::io::Error),
    VersionNumberParsingError(std::num::ParseIntError),
    ReturnedVersionNotProperlyFormattedError,
}

pub fn parse_java_version(
    java_executable: &PathBuf,
) -> Result<JavaVersion, JavaVersionParsingError> {
    let output = Command::new(&java_executable)
        .arg("--version")
        .output()
        .map_err(JavaVersionParsingError::JavaExecutionError)?;

    let binding = String::from_utf8_lossy(&output.stdout); //Avoid drop
    let capture = VERSION_MATCHER
        .captures(
            binding
                .lines()
                .next()
                .ok_or(JavaVersionParsingError::ReturnedVersionNotProperlyFormattedError)?,
        )
        .ok_or(JavaVersionParsingError::ReturnedVersionNotProperlyFormattedError)?;

    let distribution = capture[1].to_string();
    let major = capture[2]
        .parse::<u8>()
        .map_err(JavaVersionParsingError::VersionNumberParsingError)?;
    let minor = capture[3]
        .parse::<u8>()
        .map_err(JavaVersionParsingError::VersionNumberParsingError)?;
    let patch = capture[4]
        .parse::<u8>()
        .map_err(JavaVersionParsingError::VersionNumberParsingError)?;

    Ok(JavaVersion {
        distribution,
        major,
        minor,
        patch,
    })
}
