/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use egui::{Response, TextureHandle, Ui, Widget};
use web_sys::js_sys::Date;

pub struct CardWidget {
    pub thumbnail: Option<TextureHandle>,
    pub title: String,
    pub author: String,
    pub description: String,
    pub platform: String,
    pub downloads: u32,
    pub rating: f32,
    pub last_updated: f64,
}

impl CardWidget {
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

    pub fn format_downloads(downloads: u32) -> String {
        match downloads {
            d if d >= 1_000_000 => format!("{:.1}M", d as f64 / 1_000_000.0),
            d if d >= 1_000 => format!("{:.1}K", d as f64 / 1_000.0),
            _ => downloads.to_string(),
        }
    }
}

impl Widget for CardWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame::default()
            .inner_margin(egui::Margin::same(10))
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .corner_radius(egui::CornerRadius::same(10))
            .show(ui, |ui| {
                ui.set_max_width(300.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        egui::Frame::default()
                            .inner_margin(egui::Margin::same(5))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY))
                            .corner_radius(15.0)
                            .show(ui, |ui| {
                                let image_size = egui::vec2(50.0, 50.0);
                                ui.set_max_width(image_size.x);
                                ui.set_max_height(image_size.y);
                                ui.add(
                                    egui::Image::new(&self.thumbnail.unwrap_or_else(|| {
                                        ui.ctx().load_texture(
                                            "placeholder",
                                            egui::ColorImage::example(),
                                            egui::TextureOptions::default(),
                                        )
                                    }))
                                    .max_width(image_size.x)
                                    .max_height(image_size.y),
                                );
                            });
                        ui.vertical(|ui| {
                            ui.heading(&self.title);
                            ui.horizontal(|ui| {
                                ui.label("by");
                                ui.hyperlink_to(
                                    egui::RichText::new(&self.author).underline(),
                                    format!("https://example.com/author/{}", self.author),
                                );
                            });
                        });
                    });
                    ui.label(&self.description);
                    ui.label(&self.platform);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(Self::format_time_elapsed(self.last_updated));
                        ui.label(format!("★ {:.1}", self.rating.clamp(0.0, 10.0)));
                        ui.label(format!("📥 {}", Self::format_downloads(self.downloads)));
                    });
                });
            })
            .response
    }
}
