/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use eframe::{App, Frame as EframeFrame};
use egui::{
    Align, CentralPanel, Color32, Context, Frame as EguiFrame, Layout, Margin, ScrollArea,
    SidePanel, Stroke, TextEdit, TopBottomPanel, Ui, WidgetText,
};

#[derive(Default)]
enum Page {
    #[default]
    Login,
    Dashboard,
    Analytics,
    Library,
    Logout,
}

#[derive(Default)]
pub struct Xbim {
    selected_page: Page,
    username: String,
    password: String,
    login_error: Option<String>,
}

impl Xbim {
    //TODO: implement login functionality.
    fn login(&mut self, ui: &mut Ui) {
        ui.heading("Login");
        ui.label("Username:");
        ui.text_edit_singleline(&mut self.username);
        ui.label("Password:");
        ui.add(TextEdit::singleline(&mut self.password).password(true));
        if ui.button("Login").clicked() {
            if self.username == "admin" && self.password == "password" {
                self.selected_page = Page::Dashboard;
                self.login_error = None;
            } else {
                self.login_error = Some("Invalid credentials".to_owned());
            }
        }
        if let Some(ref err) = self.login_error {
            ui.colored_label(Color32::RED, err);
        }
    }

    //TODO: implement dashboard functionality.
    fn dashboard(&self, ui: &mut Ui) {
        ui.label("Dashboard Content");
    }

    //TODO: implement analytics functionality.
    fn analytics(&self, ui: &mut Ui) {
        ui.label("Analytics Content");
    }

    fn library(&self, ui: &mut Ui) {
        //TODO: replace with actual data fetching logic.
        let card_data = vec![
            ("Test1", "Description1"),
            ("Test2", "Description2"),
            ("Test3", "Description3"),
            ("Test4", "Description4"),
            ("Test5", "Description5"),
            ("Test6", "Description6"),
            ("Test7", "Description7"),
            ("Test8", "Description8"),
            ("Test9", "Description9"),
            ("Test10", "Description10"),
            ("Test11", "Description11"),
            ("Test12", "Description12"),
        ];

        ScrollArea::vertical().show(ui, |ui| {
            //TODO: fix cards not wrapping correctly. probably because card is not a widget.
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                for (title, description) in card_data {
                    self.card(ui, title, description);
                }
            });
        });
    }

    //TODO: implement logout functionality.
    fn logout(&self, ui: &mut Ui) {
        ui.label("Logout Content");
    }

    fn card(&self, ui: &mut Ui, title: impl Into<WidgetText>, description: impl Into<WidgetText>) {
        EguiFrame::default()
            .inner_margin(Margin::same(10i8))
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.label(description);
                });
            });
    }
}

impl App for Xbim {
    fn update(&mut self, ctx: &Context, _frame: &mut EframeFrame) {
        SidePanel::left("side_panel")
            //TODO: resizable doesnt work properly.
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                //TODO: make the buttons have all the same width.
                let buttons = [
                    ("Dashboard", Page::Dashboard),
                    ("Analytics", Page::Analytics),
                    ("Library", Page::Library),
                    ("Logout", Page::Logout),
                ];

                for (text, page) in buttons {
                    if ui.button(text).clicked() {
                        self.selected_page = page;
                    }
                }
            });

        CentralPanel::default().show(ctx, |ui| match self.selected_page {
            Page::Login => self.login(ui),
            Page::Dashboard => self.dashboard(ui),
            Page::Analytics => self.analytics(ui),
            Page::Library => self.library(ui),
            Page::Logout => self.logout(ui),
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            //TODO: center the copyright text.
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("© 2025 ");
                    ui.hyperlink_to("XODIUM™.", "https://xodium.com");
                    ui.label(" Open-Source (CAD) Software Company.");
                });
            });
        });
    }
}
