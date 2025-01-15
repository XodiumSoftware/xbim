/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

mod modules {
    pub mod pages;
}
mod app;
mod utils;

use wasm_bindgen::prelude::wasm_bindgen;

const HTML_CANVAS_ID: &str = "html_canvas_id";

#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebHandle {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    #[wasm_bindgen]
    pub async fn start(
        &self,
        canvas: web_sys::HtmlCanvasElement,
    ) -> Result<(), wasm_bindgen::JsValue> {
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_| Ok(Box::new(app::App::default()))),
            )
            .await
    }

    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
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
}

// fn main() {
//     WebLogger::init(log::LevelFilter::Debug).ok();
//     wasm_bindgen_futures::spawn_local(async {
//         let document = web_sys::window()
//             .expect("No window")
//             .document()
//             .expect("No document");
//
//         let canvas = document
//             .get_element_by_id(HTML_CANVAS_ID)
//             .expect(&format!("Element with id '{}' not found", HTML_CANVAS_ID))
//             .dyn_into::<web_sys::HtmlCanvasElement>()
//             .expect(&format!("{} was not a HtmlCanvasElement", HTML_CANVAS_ID));
//
//         let start_result = WebRunner::new()
//             .start(
//                 canvas,
//                 WebOptions::default(),
//                 Box::new(|_| Ok(Box::new(App::default()))),
//             )
//             .await;
//
//         if let Some(loading_text) = document.get_element_by_id("loading_text") {
//             match start_result {
//                 Ok(_) => {
//                     loading_text.remove();
//                 }
//                 Err(e) => {
//                     loading_text.set_inner_html(
//                         "<p> The app has crashed. See the developer console for details. </p>",
//                     );
//                     panic!("Failed to start eframe: {e:?}");
//                 }
//             }
//         }
//     });
// }
