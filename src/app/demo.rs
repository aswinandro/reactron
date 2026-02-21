use crate::core::geometry::Rect;
use crate::core::input::{PointerSignal, PointerState};
use crate::render::canvas2d;
use crate::theme::REACTRON_THEME;
use crate::ui::tree::{
    CrossAlign, EdgeInsets, LayoutProps, SizeSpec, UiAction, UiEvent, UiTree,
};
use crate::widgets::button::{Button, ButtonStyle};
use crate::widgets::container::{Container, ContainerStyle};
use crate::widgets::label::{Label, LabelStyle};
use crate::widgets::panel::{Panel, PanelStyle};
use crate::widgets::toggle::{Toggle, ToggleStyle};
use crate::widgets::triangle_hero::TriangleHero;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Default)]
pub struct DemoState {
    pub accent_on: bool,
    pub neon_mode: bool,
    pub clicks: u32,
    pub pointer: PointerState,
}

pub struct DemoApp {
    state: DemoState,
    ui: UiTree,
}

const KEY_TRIANGLE: &str = "triangle_hero";
const KEY_CLICK_LABEL: &str = "clicks_label";
const KEY_HINT_LABEL: &str = "hint_label";
const KEY_CONTROLS_PANEL: &str = "controls_panel";
const KEY_CTRL_TOGGLE_NEON: &str = "ctrl_toggle_neon";

impl DemoApp {
    pub fn new() -> Self {
        let mut ui = UiTree::column(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 440.0,
                height: 420.0,
            },
            14.0,
        );
        ui.set_padding(EdgeInsets::all(16.0));
        ui.set_align_items(CrossAlign::Center);
        ui.push_key_with(
            "accent_bar",
            Box::new(Container {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 6.0,
                },
                style: ContainerStyle {
                    fill: REACTRON_THEME.accent_secondary,
                    border: "#2a3350",
                    border_width: 0.0,
                },
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(6.0),
                align_self: Some(CrossAlign::Stretch),
            },
        );
        ui.push_key_with(
            KEY_TRIANGLE,
            Box::new(TriangleHero {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 300.0,
                    height: 220.0,
                },
                color: REACTRON_THEME.accent_primary,
            }),
            LayoutProps {
                width: SizeSpec::Fixed(300.0),
                height: SizeSpec::Fixed(220.0),
                align_self: Some(CrossAlign::Center),
            },
        );
        ui.push_key_with(
            "title_label",
            Box::new(Label {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 24.0,
                },
                text: "Reactron Demo Surface".to_string(),
                style: LabelStyle {
                    font: REACTRON_THEME.font_label,
                    color: REACTRON_THEME.text_primary,
                },
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(24.0),
                align_self: Some(CrossAlign::Stretch),
            },
        );
        ui.push_key_with(
            KEY_CLICK_LABEL,
            Box::new(Label {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 24.0,
                },
                text: "Button clicks: 0".to_string(),
                style: LabelStyle {
                    font: REACTRON_THEME.font_label,
                    color: REACTRON_THEME.text_muted,
                },
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(24.0),
                align_self: Some(CrossAlign::Stretch),
            },
        );
        ui.push_key_with(
            KEY_HINT_LABEL,
            Box::new(Label {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 24.0,
                },
                text: "Click/tap the button to toggle accent color".to_string(),
                style: LabelStyle {
                    font: REACTRON_THEME.font_label,
                    color: REACTRON_THEME.text_muted,
                },
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(24.0),
                align_self: Some(CrossAlign::Stretch),
            },
        );
        ui.push_key_with(
            KEY_CONTROLS_PANEL,
            Box::new(Panel {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 64.0,
                },
                style: PanelStyle {
                    fill: "#0d1324",
                    border: "#2a3350",
                    border_width: 1.0,
                },
                padding: EdgeInsets::all(10.0),
                child: {
                    let mut row = UiTree::row(
                        Rect {
                            x: 0.0,
                            y: 0.0,
                            width: 388.0,
                            height: 44.0,
                        },
                        10.0,
                    );
                    row.set_align_items(CrossAlign::Center);
                    row.push_key_with(
                        "ctrl_button_toggle_accent",
                        Box::new(Button {
                            action: UiAction::ToggleAccent,
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            label: "Reactron Button",
                            style: ButtonStyle {
                                idle_fill: "#18233d",
                                hover_fill: "#283960",
                                pressed_fill: "#1f2a47",
                                border: "#3d5387",
                                focus_border: "#27ffd8",
                                text: REACTRON_THEME.text_primary,
                                font: REACTRON_THEME.font_button,
                            },
                            focused: false,
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(2.0),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                    );
                    row.push_key_with(
                        KEY_CTRL_TOGGLE_NEON,
                        Box::new(Toggle {
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            value: true,
                            label: "Neon Mode",
                            style: ToggleStyle::default(),
                            focused: false,
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(1.0),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                    );
                    row
                },
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(64.0),
                align_self: Some(CrossAlign::Stretch),
            },
        );

        Self {
            state: DemoState {
                accent_on: true,
                neon_mode: true,
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
        let hero_color = if self.state.neon_mode {
            triangle_color
        } else {
            "#7481a3"
        };

        let interaction_text = if self.state.pointer.is_down {
            "Pointer down: release on button to trigger"
        } else {
            "Click/tap to interact, Tab to focus controls, Enter to activate"
        };

        self.ui.set_area(Rect {
            x: width * 0.5 - 220.0,
            y: height * 0.1,
            width: 440.0,
            height: height * 0.8,
        });

        if let Some(hero) = self.ui.widget_mut_by_key::<TriangleHero>(KEY_TRIANGLE) {
            hero.set_color(hero_color);
        }
        if let Some(clicks_label) = self.ui.widget_mut_by_key::<Label>(KEY_CLICK_LABEL) {
            clicks_label.set_text(format!("Button clicks: {}", self.state.clicks));
        }
        if let Some(hint_label) = self.ui.widget_mut_by_key::<Label>(KEY_HINT_LABEL) {
            hint_label.set_text(interaction_text.to_string());
        }
        if let Some(panel) = self.ui.widget_mut_by_key::<Panel>(KEY_CONTROLS_PANEL) {
            if let Some(toggle) = panel.child_mut().widget_mut_by_key::<Toggle>(KEY_CTRL_TOGGLE_NEON) {
                toggle.set_value(self.state.neon_mode);
            }
        }

        let events = self.ui.draw(context, &self.state.pointer);
        for event in events {
            match event {
                UiEvent::Action(UiAction::ToggleAccent) => {
                    self.state.accent_on = !self.state.accent_on;
                    self.state.clicks += 1;
                }
                UiEvent::Action(UiAction::SetNeon(enabled)) => {
                    self.state.neon_mode = enabled;
                }
            }
        }

        self.state.pointer.reset_transient();
        Ok(())
    }
}
