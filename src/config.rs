/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use serde::{Deserialize, Serialize};
use std::{env, error, fs, io};

/// Configuration settings for the application.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub api_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_port: 8080,
            database_url: "localhost:8000".to_string(),
            database_username: "".to_string(),
            database_password: "".to_string(),
            api_key: "".to_string(),
        }
    }
}

impl Config {
    /// Initializes the configuration from a file.
    pub fn init() -> Result<Self, Box<dyn error::Error>> {
        let exe_path = env::current_exe()?;
        let exe_dir = exe_path
            .parent()
            .ok_or("Failed to get executable directory")?;
        let config_path = exe_dir.join("config.toml");

        match fs::read_to_string(&config_path) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let default_config = Self::default();
                fs::write(&config_path, toml::to_string_pretty(&default_config)?)?;
                println!("Created new config file at: {}", config_path.display());
                Ok(default_config)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
