/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */
use std::sync::LazyLock;
use web_sys::js_sys::Date;

pub struct Utils;

pub static CARD_DATA: LazyLock<
    Vec<(
        Option<&'static str>,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        u32,
        f32,
        f64,
    )>,
> = LazyLock::new(|| {
    vec![
        (
            Some("thumbnail1.png"),
            "Test1",
            "Illyrius",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            "Windows",
            1200,
            4.5,
            1697040000000.0,
        ),
        (
            Some("thumbnail2.png"),
            "Test2",
            "Illyrius",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            "Linux",
            3400,
            4.8,
            1697030000000.0,
        ),
    ]
});

impl Utils {
    ///
    /// Formats the time elapsed since the given timestamp into a human-readable string.
    ///
    /// # Arguments
    ///
    /// * `last_updated` - A `f64` representing the timestamp (in milliseconds) of the last update.
    ///
    /// # Returns
    ///
    /// A `String` describing the time elapsed in a human-readable format, such as "⏳ updated 5 minutes ago".
    ///
    /// # Examples
    ///
    /// ```
    /// use app::utils::Utils;
    ///
    /// let formatted = Utils::format_time_elapsed(1697040000000.0);
    /// println!("{}", formatted); // Output: ⏳ updated X minutes ago
    /// ```
    pub fn format_time_elapsed(last_updated: f64) -> String {
        let elapsed_seconds = (Date::now() - last_updated) / 1000.0;
        let time_ranges = [
            (60.0, "seconds", 1.0),
            (3600.0, "minutes", 60.0),
            (86400.0, "hours", 3600.0),
            (31536000.0, "days", 86400.0),
            (31536000.0, "months", 2592000.0),
            (f64::INFINITY, "years", 31536000.0),
        ];
        for &(limit, unit, divisor) in &time_ranges {
            if elapsed_seconds < limit {
                return format!("⏳ updated {:.0} {} ago", elapsed_seconds / divisor, unit);
            }
        }
        unreachable!("Time ranges should cover all cases");
    }

    ///
    /// Formats the number of downloads into a compact, human-readable string.
    ///
    /// # Arguments
    ///
    /// * `downloads` - A `u32` representing the number of downloads.
    ///
    /// # Returns
    ///
    /// A `String` describing the number of downloads in a compact format, such as "1.2K" or "3.4M".
    ///
    /// # Examples
    ///
    /// ```
    /// use app::utils::Utils;
    ///
    /// let formatted = Utils::format_downloads(1234);
    /// println!("{}", formatted); // Output: 1.2K
    /// ```
    pub fn format_downloads(downloads: u32) -> String {
        match downloads {
            d if d >= 1_000_000 => format!("{:.1}M", d as f64 / 1_000_000.0),
            d if d >= 1_000 => format!("{:.1}K", d as f64 / 1_000.0),
            _ => downloads.to_string(),
        }
    }
}
