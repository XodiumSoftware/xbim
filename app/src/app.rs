/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use crate::style::Style;
use crate::utils::Utils;
use crate::widgets::card::CardWidget;
use eframe::{App, Frame as EframeFrame};
use egui::{
    Button, CentralPanel, Context, RichText, ScrollArea, SidePanel, TopBottomPanel, Ui,
    global_theme_preference_switch,
};
use std::fmt::{Display, Formatter, Result as FmtResult};
use web_sys::js_sys::Date;

#[derive(Default, PartialEq)]
enum Page {
    #[default]
    Dashboard,
    Analytics,
    Library,
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let text = match self {
            Page::Dashboard => "Dashboard",
            Page::Analytics => "Analytics",
            Page::Library => "Library",
        };
        write!(f, "{}", text)
    }
}

pub struct Xbim {
    selected_page: Page,
    side_panel_visible: bool,
}

impl Default for Xbim {
    fn default() -> Self {
        Self {
            selected_page: Page::Dashboard,
            side_panel_visible: true,
        }
    }
}

impl Xbim {
    //TODO: implement dashboard functionality.
    fn dashboard(&self, ui: &mut Ui) {
        ui.label("Dashboard Content");
    }

    //TODO: implement analytics functionality.
    fn analytics(&self, ui: &mut Ui) {
        ui.label("Analytics Content");
    }

    fn library(&self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.spacing_mut().item_spacing.x = Style::SPACING_M;
            ui.horizontal_wrapped(|ui| {
                for (title, description) in Utils::CARD_DATA.iter() {
                    ui.add(CardWidget {
                        thumbnail: None,
                        title: title.to_string(),
                        author: "Illyrius".to_string(),
                        description: description.to_string(),
                        platform: "Windows".to_string(),
                        downloads: 0,
                        rating: 0.0,
                        last_updated: Date::now(),
                    });
                }
            });
        });
    }
}

impl App for Xbim {
    fn update(&mut self, ctx: &Context, _frame: &mut EframeFrame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = Style::SPACING_M;
            ui.horizontal(|ui| {
                if ui.button("☰").clicked() { self.side_panel_visible = !self.side_panel_visible }
                ui.separator();
                global_theme_preference_switch(ui);
                ui.separator();
                let mut screen_reader = ui.ctx().options(|o| o.screen_reader);
                ui.checkbox(&mut screen_reader, "🔈 Screen reader").on_hover_text("Experimental feature: checking this will turn on the screen reader on supported platforms");
                ui.ctx().options_mut(|o| o.screen_reader = screen_reader);
            });
        });

        if self.side_panel_visible {
            SidePanel::left("side_panel")
                .default_width(150.0)
                .resizable(false)
                .show(ctx, |ui| {
                    for page in [Page::Dashboard, Page::Analytics, Page::Library] {
                        if ui
                            .add_sized([120.0, 30.0], Button::new(page.to_string()))
                            .clicked()
                        {
                            self.selected_page = page;
                        }
                    }
                });
        }

        CentralPanel::default().show(ctx, |ui| match self.selected_page {
            Page::Dashboard => self.dashboard(ui),
            Page::Analytics => self.analytics(ui),
            Page::Library => self.library(ui),
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            //TODO: center the copyright text.
            ui.horizontal(|ui| {
                ui.label("© 2025 ");
                ui.hyperlink_to(RichText::new("XODIUM™.").underline(), "https://xodium.org");
                ui.label(" Open-Source (CAD) Software Company.");
            });
        });
    }
}
