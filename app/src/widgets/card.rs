/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::style::Style;
use crate::utils::Utils;
use egui::{
    Align, Color32, ColorImage, CornerRadius, Frame, Image, Layout, Margin, Response, RichText,
    Shadow, Stroke, TextureHandle, TextureOptions, Ui, Widget, vec2,
};

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

impl Widget for CardWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        Frame::default()
            .inner_margin(Margin::same(Style::MARGIN_M))
            .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
            .corner_radius(CornerRadius::same(Style::ROUNDING_M))
            .shadow(Shadow::NONE)
            .show(ui, |ui| {
                ui.set_max_width(Style::WIDTH_M);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        Frame::default()
                            .inner_margin(Margin::same(Style::MARGIN_M / 2))
                            .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
                            .corner_radius(CornerRadius::same(Style::ROUNDING_M))
                            .show(ui, |ui| {
                                let image_size = vec2(Style::IMAGE_SIZE_S, Style::IMAGE_SIZE_S);
                                ui.set_max_width(image_size.x);
                                ui.set_max_height(image_size.y);
                                ui.add(
                                    Image::new(&self.thumbnail.unwrap_or_else(|| {
                                        ui.ctx().load_texture(
                                            "placeholder",
                                            ColorImage::example(),
                                            TextureOptions::default(),
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
                                    RichText::new(&self.author).underline(),
                                    // TODO: replace with propper url.
                                    format!("https://example.com/author/{}", self.author),
                                );
                            });
                        });
                    });
                    ui.label(&self.description);
                    ui.label(&self.platform);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(Utils::format_time_elapsed(self.last_updated));
                        ui.label(format!("★ {:.1}", self.rating.clamp(0.0, 10.0)));
                        ui.label(format!("📥 {}", Utils::format_downloads(self.downloads)));
                    });
                });
            })
            .response
    }
}
