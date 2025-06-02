/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

mod app;

fn main() {
    eframe::run_native(
        "xBIM",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
            Ok(Box::new(app::Xbim::default()))
        }),
    )
    .expect("Failed to run native application")
}
