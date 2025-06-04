/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use crate::app::Xbim;
use catppuccin_egui::{MOCHA, set_theme};
use eframe::{NativeOptions, run_native};

mod app;

fn main() {
    run_native(
        "xBIM",
        NativeOptions::default(),
        Box::new(|cc| {
            set_theme(&cc.egui_ctx, MOCHA);
            Ok(Box::new(Xbim::default()))
        }),
    )
    .expect("Failed to run native application")
}
