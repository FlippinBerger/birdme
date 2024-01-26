use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub region: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        // TODO try to get default region here
        Config { region: None }
    }
}

const CONFIG_FILE: &str = "~/.birdme/config.json";

// get_config reads the config file from the filesystem into a Config
pub fn get_config() -> Option<Config> {
    match fs::read_to_string(CONFIG_FILE) {
        // TODO maybe update this one to handle json parsing errors
        Ok(contents) => serde_json::from_str(&contents).unwrap(),
        Err(e) => {
            println!("Unable to read config file, {:?}", e);
            None
        }
    }
}

pub fn update_region(region: String) -> Result<(), std::io::Error> {
    let mut config = Config::new();
    config.region = Some(region);

    write_config(config)
}

fn write_config(conf: Config) -> Result<(), std::io::Error> {
    std::fs::write(CONFIG_FILE, serde_json::to_string_pretty(&conf).unwrap())
}
