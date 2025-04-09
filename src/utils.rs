/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::config::AppConfig;
use colored::*;
use std::env;
use std::path::PathBuf;
use surrealdb::Error;

/// A utility struct for common helper functions.
pub struct Utils;

impl Utils {
    /// Returns a path to a file in the same directory as the current executable.
    ///
    /// # Arguments
    /// * `filename` - The name of the file to locate (e.g., "config.toml")
    ///
    /// # Returns
    /// A `PathBuf` pointing to the specified file in the executable's directory
    pub fn get_exec_path(filename: &str) -> PathBuf {
        env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Failed to get executable directory")
            .join(filename)
    }

    /// Displays a formatted error message for database connection issues.
    ///
    /// # Arguments
    /// * `error` - The error encountered during database connection.
    /// * `config` - The application configuration containing the database URL.
    pub fn database_err_msg(error: &Error, config: &AppConfig) {
        let error_message = "DATABASE ERROR";
        let padding = 6;
        let total_width = error_message.len() + (padding * 2);
        let top = format!("╭{}╮", "─".repeat(total_width)).bright_red();
        let middle = format!(
            "│{}{}{}│",
            " ".repeat(padding),
            error_message,
            " ".repeat(padding)
        )
        .bright_red()
        .bold();
        let bottom = format!("╰{}╯", "─".repeat(total_width)).bright_red();

        eprintln!("{}\n{}\n{}", top, middle, bottom);
        eprintln!("{} {}", "● URL:".yellow().bold(), config.database_url);

        if error.to_string().contains("authentication") {
            eprintln!(
                "{} {}",
                "● Error:".yellow().bold(),
                "Authentication failed".red()
            );
            eprintln!(
                "{} {}",
                "● Note:".yellow().bold(),
                "Check your database username and password".bright_white()
            );
        } else {
            eprintln!(
                "{} {}",
                "● Problem:".yellow().bold(),
                "Connection failed".red()
            );
            eprintln!(
                "{} {}",
                "● Note:".yellow().bold(),
                "Check if SurrealDB is running and network connectivity".bright_white()
            );
        }
    }
}
