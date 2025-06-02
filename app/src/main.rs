/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use app::Xbim;
use eframe::{NativeOptions, run_native};

mod app;

fn main() {
    run_native(
        "xBIM",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Xbim::default()))),
    )
    .expect("Failed to run native application")
}
