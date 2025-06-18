/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use eframe::{App, Frame as EframeFrame};
use egui::{
    Button, CentralPanel, Context, Response, ScrollArea, SidePanel, TopBottomPanel, Ui, Widget,
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

#[derive(Default)]
pub struct Xbim {
    selected_page: Page,
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
        //TODO: replace with actual data fetching logic.
        let card_data = vec![
            (
                "Test1",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test2",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test3",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test4",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test5",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test6",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test7",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test8",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test9",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test10",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test11",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
            (
                "Test12",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            ),
        ];

        ScrollArea::vertical().show(ui, |ui| {
            //TODO: fix cards not wrapping correctly. probably because card is not a widget.
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                for (title, description) in card_data {
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
        SidePanel::left("side_panel")
            //TODO: resizable doesnt work properly.
            .default_width(150.0)
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

        CentralPanel::default().show(ctx, |ui| match self.selected_page {
            Page::Dashboard => self.dashboard(ui),
            Page::Analytics => self.analytics(ui),
            Page::Library => self.library(ui),
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            //TODO: center the copyright text.
            ui.horizontal(|ui| {
                ui.label("© 2025 ");
                ui.hyperlink_to(
                    egui::RichText::new("XODIUM™.").underline(),
                    "https://xodium.org",
                );
                ui.label(" Open-Source (CAD) Software Company.");
            });
        });
    }
}

struct CardWidget {
    thumbnail: Option<egui::TextureHandle>,
    title: String,
    author: String,
    description: String,
    platform: String,
    downloads: u32,
    rating: f32,
    last_updated: f64,
}

impl Widget for CardWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame::default()
            .inner_margin(egui::Margin::same(10))
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .corner_radius(egui::CornerRadius::same(10))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    if let Some(thumbnail) = self.thumbnail {
                        ui.image(&thumbnail);
                    }
                    ui.horizontal(|ui| {
                        ui.heading(&self.title);
                        ui.label("by");
                        ui.hyperlink_to(
                            egui::RichText::new(&self.author).underline(),
                            format!("https://example.com/author/{}", self.author),
                        );
                    });
                    ui.label(&self.description);
                    ui.label(&self.platform);
                    ui.label(format!("📥 {}", self.downloads));
                    ui.label(format!("★ {:.1}", self.rating.clamp(0.0, 10.0)));
                    ui.label(format!("{:.0} ms ago", self.last_updated));
                });
            })
            .response
    }
}
