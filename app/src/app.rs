/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

#[derive(Default)]
enum Page {
    #[default]
    Dashboard,
    Analytics,
    Library,
    Logout,
}

#[derive(Default)]
pub struct Xbim {
    selected_page: Page,
}

impl Xbim {
    //TODO: implement dashboard functionality.
    fn dashboard(&self, ui: &mut egui::Ui) {
        ui.label("Dashboard Content");
    }

    //TODO: implement analytics functionality.
    fn analytics(&self, ui: &mut egui::Ui) {
        ui.label("Analytics Content");
    }

    fn library(&self, ui: &mut egui::Ui) {
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

        egui::ScrollArea::vertical().show(ui, |ui| {
            //TODO: fix cards not wrapping correctly. probably because card is not a widget.
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                for (title, description) in card_data {
                    self.card(ui, title, description);
                }
            });
        });
    }

    //TODO: implement login functionality.
    //TODO: implement logout functionality.
    fn logout(&self, ui: &mut egui::Ui) {
        ui.label("Logout Content");
    }

    fn card(
        &self,
        ui: &mut egui::Ui,
        title: impl Into<egui::WidgetText>,
        description: impl Into<egui::WidgetText>,
    ) {
        egui::Frame::default()
            .inner_margin(egui::Margin::same(10i8))
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.label(description);
                });
            });
    }
}

impl eframe::App for Xbim {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel")
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

        egui::CentralPanel::default().show(ctx, |ui| match self.selected_page {
            Page::Dashboard => self.dashboard(ui),
            Page::Analytics => self.analytics(ui),
            Page::Library => self.library(ui),
            Page::Logout => self.logout(ui),
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            //TODO: center the copyright text.
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("© 2025 ");
                    ui.hyperlink_to("XODIUM™.", "https://xodium.com");
                    ui.label(" Open-Source (CAD) Software Company.");
                });
            });
        });
    }
}
