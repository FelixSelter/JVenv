use lazy_static::lazy_static;
use std::{
    fs::{self, create_dir_all, File},
    path::PathBuf,
};

use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct JavaHome {
    pub path: PathBuf,
    pub name: String,
}
impl JavaHome {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self { path, name }
    }
}

#[derive(Serialize, Default, Deserialize, Clone)]
pub struct Config {
    pub java_homes: Vec<JavaHome>,
}

lazy_static! {
    static ref CONFIG_PATH: PathBuf = {
        let mut config_path = config_dir().unwrap();
        config_path.push("JVEnv");
        create_dir_all(config_path.clone()).unwrap();
        config_path.push("config.json");
        config_path
    };
}

impl Config {
    pub fn load() -> Self {
        File::open(CONFIG_PATH.as_path())
            .ok()
            .and_then(|file| serde_json::from_reader::<File, Config>(file).ok())
            .unwrap_or_default()

        //TODO: Validate the config. Check that all java_homes are still intact
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        fs::write(CONFIG_PATH.as_path(), serde_json::to_string(self).unwrap())
    }
}
