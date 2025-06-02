/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use eframe::{App, Frame, NativeOptions, run_native};
use egui::{CentralPanel, Context};

struct Xbim;

impl App for Xbim {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, egui!");
        });
    }
}

fn main() {
    run_native(
        "xBIM",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Xbim))),
    )
    .expect("TODO: panic message")
}
