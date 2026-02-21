mod app;
mod core;
mod platform;
mod render;
mod theme;
mod ui;
mod widgets;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    platform::web::start()
}
