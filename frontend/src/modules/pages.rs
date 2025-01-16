/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::utils::Utils;
use egui::{CentralPanel, Context, RichText};

#[derive(PartialEq)]
pub enum Page {
    Home,
    ControlPanel,
}
pub struct Pages {}

impl Pages {
    // `Home` page constants.
    const HOME_TITLE: &'static str = "Xodium";
    const HOME_TITLE_COLOR: &'static str = "#CB2D3E";
    const HOME_DESC: &'static str = "Open-Source (CAD) Software Company";

    // `ControlPanel` page constants.
    const CONTROL_PANEL_TITLE: &'static str = "Control Panel";

    /// Displays the `Home` page.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The `Context` struct.
    pub fn home(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new(Self::HOME_TITLE)
                        .color(Utils::hex_color(Self::HOME_TITLE_COLOR))
                        .strong(),
                );
                ui.label(Self::HOME_DESC);
            });
        });
    }

    /// Displays the `ControlPanel` page.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The `Context` struct.
    pub fn control_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(Self::CONTROL_PANEL_TITLE);
                // TODO: Add control panel UI elements here
            });
        });
    }
}
