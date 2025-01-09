/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

mod modules {
    pub mod auth;
    pub mod pages;
}
mod app;
mod utils;

use crate::app::App;
use eframe::{WebLogger, WebOptions, WebRunner};
use wasm_bindgen::JsCast as _;

const HTML_CANVAS_ID: &str = "html_canvas_id";

fn main() {
    WebLogger::init(log::LevelFilter::Debug).ok();
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id(HTML_CANVAS_ID)
            .expect(&format!("Element with id '{}' not found", HTML_CANVAS_ID))
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect(&format!("{} was not a HtmlCanvasElement", HTML_CANVAS_ID));

        let start_result = WebRunner::new()
            .start(
                canvas,
                WebOptions::default(),
                Box::new(|_| Ok(Box::new(App::default()))),
            )
            .await;

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
