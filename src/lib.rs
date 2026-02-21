mod app;
mod core;
mod platform;
mod render;
mod widgets;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    platform::web::start()
}
