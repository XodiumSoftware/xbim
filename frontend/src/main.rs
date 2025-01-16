/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::app::App;
use eframe::{WebLogger, WebOptions, WebRunner};
use log::LevelFilter;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlCanvasElement};

mod modules {
    pub mod pages;
}
mod app;
mod utils;

const HTML_CANVAS_ID: &str = "html_canvas_id";
const LOADING_TEXT_ID: &str = "loading_text";

fn main() {
    WebLogger::init(LevelFilter::Debug).ok();
    spawn_local(async {
        let document = window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id(HTML_CANVAS_ID)
            .expect(&format!("Element with id '{}' not found", HTML_CANVAS_ID))
            .dyn_into::<HtmlCanvasElement>()
            .expect(&format!("{} was not a HtmlCanvasElement", HTML_CANVAS_ID));

        let start_result = WebHandle::new().start(canvas).await;

        if let Some(loading_text) = document.get_element_by_id(LOADING_TEXT_ID) {
            match start_result {
                Ok(_) => loading_text.remove(),
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

#[wasm_bindgen]
pub struct WebHandle {
    runner: WebRunner,
    initialized: bool,
}

#[wasm_bindgen]
impl WebHandle {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        WebLogger::init(LevelFilter::Debug).ok();
        Self {
            runner: WebRunner::new(),
            initialized: false,
        }
    }

    #[wasm_bindgen]
    pub async fn start(&mut self, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
        let result = self
            .runner
            .start(
                canvas,
                WebOptions::default(),
                Box::new(|_| Ok(Box::new(App::default()))),
            )
            .await;
        self.initialized = result.is_ok();
        result
    }

    #[wasm_bindgen]
    pub fn destroy(&mut self) {
        self.runner.destroy();
        self.initialized = false;
    }

    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.message())
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.callstack())
    }

    #[wasm_bindgen]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
