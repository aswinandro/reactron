use crate::core::geometry::Rect;
use crate::core::input::{PointerSignal, PointerState};
use crate::render::canvas2d;
use crate::theme::REACTRON_THEME;
use crate::ui::tree::{UiAction, UiEvent, UiTree};
use crate::widgets::button::{Button, ButtonStyle};
use crate::widgets::container::{Container, ContainerStyle};
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
    ui: UiTree,
}

const WIDGET_TRIANGLE: usize = 1;
const WIDGET_CLICK_LABEL: usize = 2;
const WIDGET_HINT_LABEL: usize = 3;

impl DemoApp {
    pub fn new() -> Self {
        let mut ui = UiTree::new(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 420.0,
            },
            14.0,
        );
        ui.push(Box::new(Container {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 420.0,
            },
            style: ContainerStyle {
                fill: "#0d1324",
                border: "#2a3350",
                border_width: 1.0,
            },
        }));
        ui.push(Box::new(TriangleHero {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 250.0,
            },
            color: REACTRON_THEME.accent_primary,
        }));
        ui.push(Box::new(Label {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 24.0,
            },
            text: "Button clicks: 0".to_string(),
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
            text: "Click/tap the button to toggle accent color".to_string(),
            style: LabelStyle {
                font: REACTRON_THEME.font_label,
                color: REACTRON_THEME.text_muted,
            },
        }));
        ui.push(Box::new(Button {
            action: UiAction::ToggleAccent,
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

        Self {
            state: DemoState {
                accent_on: true,
                ..DemoState::default()
            },
            ui,
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

        self.ui.set_area(Rect {
            x: width * 0.5 - 220.0,
            y: height * 0.14,
            width: 440.0,
            height: 420.0,
        });

        if let Some(hero) = self.ui.widget_mut::<TriangleHero>(WIDGET_TRIANGLE) {
            hero.set_color(triangle_color);
        }
        if let Some(clicks_label) = self.ui.widget_mut::<Label>(WIDGET_CLICK_LABEL) {
            clicks_label.set_text(format!("Button clicks: {}", self.state.clicks));
        }
        if let Some(hint_label) = self.ui.widget_mut::<Label>(WIDGET_HINT_LABEL) {
            hint_label.set_text(interaction_text.to_string());
        }

        let events = self.ui.draw(context, &self.state.pointer);
        for event in events {
            match event {
                UiEvent::Action(UiAction::ToggleAccent) => {
                    self.state.accent_on = !self.state.accent_on;
                    self.state.clicks += 1;
                }
            }
        }

        self.state.pointer.reset_transient();
        Ok(())
    }
}
