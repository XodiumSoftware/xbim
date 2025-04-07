/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::serde::uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};

/// Configuration settings for the application.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub api_key: Uuid,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "db.xodium.org".to_string(),
            database_username: String::new(),
            database_password: String::new(),
            api_key: Uuid::now_v7(),
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}
