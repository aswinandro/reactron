use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn draw_scene(context: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement, primary_color: &str) {
    context.set_fill_style_str("#080b13");
    context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    context.begin_path();
    context.move_to(400.0, 120.0);
    context.line_to(220.0, 420.0);
    context.line_to(580.0, 420.0);
    context.close_path();
    context.set_fill_style_str(primary_color);
    context.fill();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window not available"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document not available"))?;

    let canvas = document
        .get_element_by_id("reactron-canvas")
        .ok_or_else(|| JsValue::from_str("canvas #reactron-canvas not found"))?
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("2d context unavailable"))?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let is_red = Rc::new(Cell::new(true));
    draw_scene(&context, &canvas, "#ff2d2d");

    let canvas_for_click = canvas.clone();
    let context_for_click = context.clone();
    let is_red_for_click = Rc::clone(&is_red);
    let on_click = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
        let next_is_red = !is_red_for_click.get();
        is_red_for_click.set(next_is_red);
        let color = if next_is_red { "#ff2d2d" } else { "#27ffd8" };
        draw_scene(&context_for_click, &canvas_for_click, color);
    });

    canvas.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())?;
    on_click.forget();

    Ok(())
}
