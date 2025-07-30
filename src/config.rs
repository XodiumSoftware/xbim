#![warn(clippy::all)]
#![forbid(unsafe_code)]

use crate::utils::Utils;
use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use rand::Rng;
use rand::distr::Alphanumeric;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Configuration settings for the application.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub secret_key: String,
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,
    pub tls_cert_path: String,
    pub tls_key_path: String,
}

impl Config {
    /// Creates a new instance of `Config` with default values.
    ///
    /// # Returns
    /// A `Self` instance containing the default configuration.
    pub fn new() -> Self {
        Self::load_or_create(&Utils::get_exec_path("config.toml"))
    }

    /// Loads the configuration from a file, creating a default one if it doesn't exist.
    ///
    /// # Arguments
    /// * `path` - The path to the configuration file.
    ///
    /// # Returns
    /// A `Self` instance containing the loaded or default configuration.
    pub fn load_or_create(path: &PathBuf) -> Self {
        let mut config = if path.exists() {
            Figment::from(Serialized::defaults(Self::default()))
                .merge(Toml::file(path))
                .extract::<Self>()
                .unwrap_or_else(|err| {
                    eprintln!("Configuration error (using defaults): {err}");
                    Self::default()
                })
        } else {
            println!("Creating default config at: {}", path.display());
            Self::default()
        };

        if config.secret_key.is_empty() {
            config.generate_secret_key();
            config.save_to_file(path).unwrap_or_else(|err| {
                eprintln!("Failed to update config with new secret key: {err}")
            });
        }

        config
    }

    /// Generates a new random secret key if the current one is empty.
    /// # Note
    /// - This only generates a key if `secret_key` is currently empty
    /// - The generated key is printed to stdout for visibility
    /// - For production use, consider:
    ///   - Storing the key securely (not in plaintext in config files)
    ///   - Using a dedicated secrets management system
    fn generate_secret_key(&mut self) {
        if self.secret_key.is_empty() {
            self.secret_key = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();
            println!("Generated new secret key");
        }
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
