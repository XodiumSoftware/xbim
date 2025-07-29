#![warn(clippy::all)]
#![forbid(unsafe_code)]

use crate::config::Config;
use colored::*;
use std::path::PathBuf;
use std::{env, path};
use surrealdb::Error;

/// A utility struct for common helper functions.
pub struct Utils;

impl Utils {
    /// Returns a path to a file relative to the current executable's directory.
    ///
    /// # Arguments
    /// * `path_components` - One or more path components (folders and/or filename)
    ///
    /// # Returns
    /// A `PathBuf` pointing to the specified path relative to the executable's directory
    pub fn get_exec_path<P: AsRef<path::Path>>(path_components: P) -> PathBuf {
        let mut path = env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Failed to get executable directory")
            .to_path_buf();

        path.push(path_components);
        path
    }

    /// Displays a formatted error message for database connection issues.
    ///
    /// # Arguments
    /// * `error` - The error encountered during database connection.
    /// * `config` - The application configuration containing the database URL.
    pub fn database_err_msg(error: &Error, config: &Config) {
        const ERROR_TITLE: &str = "DATABASE ERROR";
        const PADDING: usize = 6;
        const BULLET: &str = "● ";
        const LABELS: [&str; 3] = ["URL:", "Error:", "Note:"];

        let total_width = ERROR_TITLE.len() + (PADDING * 2);
        let border_line = "─".repeat(total_width);
        let box_parts = [
            format!("╭{border_line}╮"),
            format!(
                "│{}{}{}│",
                " ".repeat(PADDING),
                ERROR_TITLE,
                " ".repeat(PADDING)
            ),
            format!("╰{border_line}╯"),
        ];

        for (i, part) in box_parts.iter().enumerate() {
            eprintln!(
                "{}",
                if i == 1 {
                    part.bright_red().bold()
                } else {
                    part.bright_red()
                }
            );
        }

        eprintln!(
            "{} {}",
            format!("{BULLET} {}", LABELS[0]).yellow().bold(),
            config.database_url
        );

        let (problem, note) = if error.to_string().contains("authentication") {
            (
                "Authentication failed",
                "Check your database username and password in config.toml",
            )
        } else {
            (
                "Connection failed",
                "Check if SurrealDB is running and network connectivity",
            )
        };

        eprintln!(
            "{} {}",
            format!("{BULLET} {}", LABELS[1]).yellow().bold(),
            problem.red()
        );
        eprintln!(
            "{} {}",
            format!("{BULLET} {}", LABELS[2]).yellow().bold(),
            note.bright_white()
        );
    }
}
