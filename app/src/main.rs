/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use app::Xbim;
use eframe::wasm_bindgen::JsCast as _;
use eframe::{WebLogger, WebOptions, WebRunner};
use log::LevelFilter;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, window};

mod app;
mod style;
mod utils;

mod widgets {
    pub mod card;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    WebLogger::init(LevelFilter::Debug).ok();

    spawn_local(async {
        let document = window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = WebRunner::new()
            .start(
                canvas,
                WebOptions::default(),
                Box::new(|_cc| Ok(Box::new(Xbim::default()))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
