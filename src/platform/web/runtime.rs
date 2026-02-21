use crate::app::demo::DemoApp;
use crate::core::input::PointerSignal;
use crate::render::canvas2d;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, PointerEvent, WheelEvent, Window};

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

    let app = Rc::new(RefCell::new(DemoApp::new()));
    let _ = canvas.set_attribute("tabindex", "0");

    render_now(&app, &context, &canvas, &window)?;

    {
        let app_ref = Rc::clone(&app);
        let context_ref = context.clone();
        let canvas_ref = canvas.clone();
        let window_ref = window.clone();
        let on_keydown = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
            let key = event.key();
            if (event.ctrl_key() || event.meta_key()) && key.eq_ignore_ascii_case("a") {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::SelectAll,
                );
                return;
            }
            if key == "Enter" || key == " " {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::ActivatePrimary,
                );
            } else if key == "Tab" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::FocusNext,
                );
            } else if key == "Backspace" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::Backspace,
                );
            } else if key == "Delete" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::DeleteForward,
                );
            } else if key == "ArrowLeft" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    if event.shift_key() {
                        PointerSignal::MoveLeftSelect
                    } else {
                        PointerSignal::MoveLeft
                    },
                );
            } else if key == "ArrowRight" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    if event.shift_key() {
                        PointerSignal::MoveRightSelect
                    } else {
                        PointerSignal::MoveRight
                    },
                );
            } else if key == "ArrowUp" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::MoveUp,
                );
            } else if key == "ArrowDown" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::MoveDown,
                );
            } else if key == "Home" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::MoveHome,
                );
            } else if key == "End" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::MoveEnd,
                );
            } else if key == "Escape" {
                event.prevent_default();
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::Cancel,
                );
            } else if key.len() == 1 && !event.ctrl_key() && !event.meta_key() {
                dispatch_and_render(
                    &app_ref,
                    &context_ref,
                    &canvas_ref,
                    &window_ref,
                    PointerSignal::TextInput(key),
                );
            }
        });
        window.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())?;
        on_keydown.forget();
    }

    {
        let app_ref = Rc::clone(&app);
        let context_ref = context.clone();
        let canvas_ref = canvas.clone();
        let window_ref = window.clone();
        let on_move = Closure::<dyn FnMut(_)>::new(move |event: PointerEvent| {
            let (x, y) = canvas2d::pointer_position_in_canvas(&event, &canvas_ref);
            dispatch_and_render(
                &app_ref,
                &context_ref,
                &canvas_ref,
                &window_ref,
                PointerSignal::Move { x, y },
            );
        });
        canvas.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref())?;
        on_move.forget();
    }

    {
        let app_ref = Rc::clone(&app);
        let context_ref = context.clone();
        let canvas_ref = canvas.clone();
        let window_ref = window.clone();
        let on_down = Closure::<dyn FnMut(_)>::new(move |event: PointerEvent| {
            let (x, y) = canvas2d::pointer_position_in_canvas(&event, &canvas_ref);
            dispatch_and_render(
                &app_ref,
                &context_ref,
                &canvas_ref,
                &window_ref,
                PointerSignal::Down { x, y },
            );
        });
        canvas.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())?;
        on_down.forget();
    }

    {
        let app_ref = Rc::clone(&app);
        let context_ref = context.clone();
        let canvas_ref = canvas.clone();
        let window_ref = window.clone();
        let on_up = Closure::<dyn FnMut(_)>::new(move |event: PointerEvent| {
            let (x, y) = canvas2d::pointer_position_in_canvas(&event, &canvas_ref);
            dispatch_and_render(
                &app_ref,
                &context_ref,
                &canvas_ref,
                &window_ref,
                PointerSignal::Up { x, y },
            );
        });
        canvas.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref())?;
        canvas.add_event_listener_with_callback("pointercancel", on_up.as_ref().unchecked_ref())?;
        on_up.forget();
    }

    {
        let app_ref = Rc::clone(&app);
        let context_ref = context.clone();
        let canvas_ref = canvas.clone();
        let window_ref = window.clone();
        let on_leave = Closure::<dyn FnMut(_)>::new(move |_event: PointerEvent| {
            dispatch_and_render(
                &app_ref,
                &context_ref,
                &canvas_ref,
                &window_ref,
                PointerSignal::Leave,
            );
        });
        canvas.add_event_listener_with_callback("pointerleave", on_leave.as_ref().unchecked_ref())?;
        on_leave.forget();
    }

    {
        let app_ref = Rc::clone(&app);
        let context_ref = context.clone();
        let canvas_ref = canvas.clone();
        let window_ref = window.clone();
        let on_wheel = Closure::<dyn FnMut(_)>::new(move |event: WheelEvent| {
            event.prevent_default();
            let (x, y) = canvas2d::client_position_in_canvas(
                f64::from(event.client_x()),
                f64::from(event.client_y()),
                &canvas_ref,
            );
            dispatch_and_render(
                &app_ref,
                &context_ref,
                &canvas_ref,
                &window_ref,
                PointerSignal::Scroll {
                    x,
                    y,
                    delta_y: event.delta_y(),
                },
            );
        });
        canvas.add_event_listener_with_callback("wheel", on_wheel.as_ref().unchecked_ref())?;
        on_wheel.forget();
    }

    Ok(())
}

fn render_now(
    app: &Rc<RefCell<DemoApp>>,
    context: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    window: &Window,
) -> Result<(), JsValue> {
    let mut app_ref = app.borrow_mut();
    app_ref.render(context, canvas, window.device_pixel_ratio())
}

fn dispatch_and_render(
    app: &Rc<RefCell<DemoApp>>,
    context: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    window: &Window,
    signal: PointerSignal,
) {
    let result = {
        let mut app_ref = app.borrow_mut();
        app_ref.handle_pointer(signal);
        app_ref.render(context, canvas, window.device_pixel_ratio())
    };

    if let Err(error) = result {
        web_sys::console::error_1(&error);
    }
}
