/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */
use figment::providers::{Format, Serialized, Toml};
use figment::Figment;
use rocket::serde::uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Configuration settings for the application.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub api_key: Uuid,
    pub tls_cert_path: String,
    pub tls_key_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "db.xodium.org".to_string(),
            database_username: String::new(),
            database_password: String::new(),
            api_key: Uuid::now_v7(),
            tls_cert_path: String::new(),
            tls_key_path: String::new(),
        }
    }
}

impl AppConfig {
    /// Loads the configuration from a file, creating a default one if it doesn't exist.
    ///
    /// # Arguments
    /// * `path` - The path to the configuration file.
    ///
    /// # Returns
    /// A `Self` instance containing the loaded or default configuration.
    pub fn load_or_create(path: &PathBuf) -> Self {
        if !path.exists() {
            println!("Creating default config at: {}", path.display());
            Self::default()
                .save_to_file(path)
                .unwrap_or_else(|err| eprintln!("Failed to create config: {}", err));
        }

        Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file(path))
            .extract::<Self>()
            .unwrap_or_else(|err| {
                eprintln!("Configuration error (using defaults): {}", err);
                Self::default()
            })
    }

    /// Saves the current configuration to a file.
    ///
    /// # Arguments
    /// * `path` - The path to the configuration file.
    ///
    /// # Returns
    /// A `std::io::Result<()>` indicating success or failure.
    pub fn save_to_file(&self, path: &PathBuf) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        File::create(path)?.write_all(
            toml::to_string_pretty(self)
                .expect("Failed to serialize config to TOML")
                .as_bytes(),
        )
    }
}
