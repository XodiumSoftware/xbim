/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

enum Page {
    Dashboard,
    Analytics,
    Library,
    Logout,
}

pub struct Xbim {
    selected_button: String,
}

impl Default for Xbim {
    fn default() -> Self {
        Self {
            selected_button: "Dashboard".to_string(),
        }
    }
}

impl Xbim {
    fn dashboard(&self, ui: &mut egui::Ui) {
        ui.label("Dashboard Content");
    }

    fn analytics(&self, ui: &mut egui::Ui) {
        ui.label("Analytics Content");
    }

    fn library(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                self.card(ui, "Test1", "Description1");
                self.card(ui, "Test2", "Description2");
                self.card(ui, "Test3", "Description3");
                self.card(ui, "Test4", "Description4");
                self.card(ui, "Test5", "Description5");
                self.card(ui, "Test6", "Description6");
                self.card(ui, "Test7", "Description7");
                self.card(ui, "Test8", "Description8");
                self.card(ui, "Test9", "Description9");
                self.card(ui, "Test10", "Description10");
                self.card(ui, "Test11", "Description11");
                self.card(ui, "Test12", "Description12");
            });
        });
    }

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
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                if ui.button("Dashboard").clicked() {
                    self.selected_button = "Dashboard".to_string();
                }
                if ui.button("Analytics").clicked() {
                    self.selected_button = "Analytics".to_string();
                }
                if ui.button("Library").clicked() {
                    self.selected_button = "Library".to_string();
                }
                if ui.button("Logout").clicked() {
                    self.selected_button = "Logout".to_string();
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.selected_button.as_str() {
            "Dashboard" => self.dashboard(ui),
            "Analytics" => self.analytics(ui),
            "Library" => self.library(ui),
            "Logout" => self.logout(ui),
            _ => self.dashboard(ui),
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
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
