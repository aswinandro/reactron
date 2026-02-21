use crate::core::input::{PointerSignal, PointerState};
use crate::core::layout::VerticalLayout;
use crate::render::canvas2d;
use crate::widgets::button::{Button, ButtonStyle};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Default)]
pub struct DemoState {
    pub accent_on: bool,
    pub clicks: u32,
    pub pointer: PointerState,
}

pub struct DemoApp {
    state: DemoState,
}

impl DemoApp {
    pub fn new() -> Self {
        Self {
            state: DemoState {
                accent_on: true,
                ..DemoState::default()
            },
        }
    }

    pub fn handle_pointer(&mut self, signal: PointerSignal) {
        self.state.pointer.apply(signal);
    }

    pub fn render(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        dpr: f64,
    ) -> Result<(), JsValue> {
        let (width, height) = canvas2d::sync_canvas_resolution(canvas, dpr);

        canvas2d::clear(context, width, height, "#080b13");
        let triangle_color = if self.state.accent_on {
            "#ff2d2d"
        } else {
            "#27ffd8"
        };
        canvas2d::draw_triangle(context, width, height, triangle_color);

        let mut layout = VerticalLayout::new(width * 0.5 - 160.0, height * 0.74, 320.0, 16.0);
        let button = Button {
            rect: layout.next(60.0),
            label: "Reactron Button",
            style: ButtonStyle::default(),
        };
        let button_state = button.draw(context, &self.state.pointer);

        if button_state.clicked {
            self.state.accent_on = !self.state.accent_on;
            self.state.clicks += 1;
        }

        canvas2d::draw_centered_text(
            context,
            &format!("Button clicks: {}", self.state.clicks),
            width / 2.0,
            height - 52.0,
            "14px Consolas",
            "#9eb4ff",
        );

        let interaction_text = if button_state.pressed {
            "Pointer down: press and release on button"
        } else if button_state.hovered {
            "Pointer over button"
        } else {
            "Click/tap the button to toggle accent color"
        };
        canvas2d::draw_centered_text(
            context,
            interaction_text,
            width / 2.0,
            height - 28.0,
            "14px Consolas",
            "#9eb4ff",
        );

        self.state.pointer.reset_transient();
        Ok(())
    }
}

