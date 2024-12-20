#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]
mod app;

use serde::{Deserialize, Serialize};
use yew::Renderer;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn main() {
    Renderer::<App>::new().render();
}
