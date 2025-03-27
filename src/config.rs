/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::Path};

/// Configuration settings for the application.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub api_key: String,
}

impl Config {
    /// Returns a new instance of the configuration with default values.
    pub fn default() -> Self {
        Self {
            server_port: 8080,
            database_url: "localhost:8000".to_string(),
            database_username: "root".to_string(),
            database_password: "root".to_string(),
            api_key: "xBIM-api-key-2025".to_string(),
        }
    }

    /// Returns the path to the configuration file.
    pub fn get_config_path() -> std::path::PathBuf {
        match dirs::config_dir() {
            Some(config_dir) => config_dir.join("xbim").join("config.toml"),
            None => Path::new("config.toml").to_path_buf(),
        }
    }

    /// Initializes the configuration by reading from the configuration file.
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let default_config = Self::default();
                fs::write(config_path, toml::to_string_pretty(&default_config)?)?;
                println!("Created new config file with default settings");
                Ok(default_config)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Saves the configuration to the configuration file.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(Self::get_config_path(), toml::to_string_pretty(self)?)?;
        Ok(())
    }
}
