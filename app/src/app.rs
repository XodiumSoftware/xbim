/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use eframe::{App, Frame as EframeFrame};
use egui::{
    Align, Button, CentralPanel, Color32, Context, Frame as EguiFrame, Layout, Margin, ScrollArea,
    SidePanel, Stroke, TopBottomPanel, Ui, WidgetText,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

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
