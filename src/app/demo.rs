use crate::core::geometry::Rect;
use crate::core::input::{PointerSignal, PointerState};
use crate::render::canvas2d;
use crate::theme::REACTRON_THEME;
use crate::ui::tree::{UiEvent, UiTree};
use crate::widgets::button::{Button, ButtonStyle};
use crate::widgets::label::{Label, LabelStyle};
use crate::widgets::triangle_hero::TriangleHero;
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

        canvas2d::clear(context, width, height, REACTRON_THEME.background);
        let triangle_color = if self.state.accent_on {
            REACTRON_THEME.accent_primary
        } else {
            REACTRON_THEME.accent_secondary
        };

        let interaction_text = if self.state.pointer.is_down {
            "Pointer down: release on button to trigger"
        } else {
            "Click/tap the button to toggle accent color"
        };

        let mut ui = UiTree::new(
            Rect {
                x: width * 0.5 - 220.0,
                y: height * 0.14,
                width: 440.0,
                height: 0.0,
            },
            14.0,
        );
        ui.push(Box::new(TriangleHero {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 280.0,
            },
            color: triangle_color,
        }));
        ui.push(Box::new(Label {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 24.0,
            },
            text: format!("Button clicks: {}", self.state.clicks),
            style: LabelStyle {
                font: REACTRON_THEME.font_label,
                color: REACTRON_THEME.text_muted,
            },
        }));
        ui.push(Box::new(Label {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 24.0,
            },
            text: interaction_text.to_string(),
            style: LabelStyle {
                font: REACTRON_THEME.font_label,
                color: REACTRON_THEME.text_muted,
            },
        }));
        ui.push(Box::new(Button {
            action: "toggle_accent",
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 60.0,
            },
            label: "Reactron Button",
            style: ButtonStyle {
                idle_fill: "#18233d",
                hover_fill: "#283960",
                pressed_fill: "#1f2a47",
                border: "#3d5387",
                text: REACTRON_THEME.text_primary,
                font: REACTRON_THEME.font_button,
            },
        }));

        let events = ui.draw(context, &self.state.pointer);
        for event in events {
            match event {
                UiEvent::Action("toggle_accent") => {
                    self.state.accent_on = !self.state.accent_on;
                    self.state.clicks += 1;
                }
                UiEvent::Action(_) => {}
            }
        }

        self.state.pointer.reset_transient();
        Ok(())
    }
}
