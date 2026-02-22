use crate::core::geometry::Rect;
use crate::core::input::{PointerSignal, PointerState};
use crate::render::canvas2d;
use crate::theme::REACTRON_THEME;
use crate::ui::tree::{
    CrossAlign, EdgeInsets, LayoutProps, SizeSpec, UiAction, UiEvent, UiTree, Widget,
};
use crate::widgets::button::{Button, ButtonStyle};
use crate::widgets::checkbox::{Checkbox, CheckboxStyle};
use crate::widgets::container::{Container, ContainerStyle};
use crate::widgets::form_field::{FormField, FormFieldStyle};
use crate::widgets::label::{Label, LabelStyle};
use crate::widgets::list_view::{ListView, ListViewStyle};
use crate::widgets::modal::{Modal, ModalStyle};
use crate::widgets::radio_group::{RadioGroup, RadioGroupStyle};
use crate::widgets::select::{Select, SelectStyle};
use crate::widgets::slider::{Slider, SliderStyle};
use crate::widgets::tabs::{Tabs, TabsStyle};
use crate::widgets::text_input::{TextInput, TextInputStyle};
use crate::widgets::toggle::{Toggle, ToggleStyle};
use crate::widgets::triangle_hero::TriangleHero;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Default)]
pub struct DemoState {
    pub accent_on: bool,
    pub neon_mode: bool,
    pub clicks: u32,
    pub query: String,
    pub preset: String,
    pub intensity: f64,
    pub animations: bool,
    pub density: String,
    pub active_tab: String,
    pub modal_result: String,
    pub show_modal: bool,
    pub selected_item: String,
    pub pointer: PointerState,
}

pub struct DemoApp {
    state: DemoState,
    ui: UiTree,
    modal: Modal,
}

const KEY_TRIANGLE: &str = "triangle_hero";
const KEY_CLICK_LABEL: &str = "clicks_label";
const KEY_HINT_LABEL: &str = "hint_label";
const KEY_TABS: &str = "main_tabs";
const KEY_RESULTS_LIST: &str = "results_list";
const KEY_CONTROLS_FIELD: &str = "controls_field";
const KEY_CTRL_QUERY: &str = "ctrl_query";
const KEY_CTRL_TOGGLE_NEON: &str = "ctrl_toggle_neon";
const KEY_CTRL_PRESET: &str = "ctrl_preset";
const KEY_CTRL_INTENSITY: &str = "ctrl_intensity";
const KEY_CTRL_ANIMATIONS: &str = "ctrl_animations";
const KEY_CTRL_DENSITY: &str = "ctrl_density";
const KEY_CTRL_MODAL_BTN: &str = "ctrl_modal_btn";

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
        ui.push_key_with_order(
            KEY_TABS,
            Box::new(Tabs {
                key: "main_tab",
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 34.0,
                },
                options: vec![
                    "Overview".to_string(),
                    "Controls".to_string(),
                    "Metrics".to_string(),
                ],
                selected: 0,
                focused: false,
                style: TabsStyle::default(),
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(34.0),
                align_self: Some(CrossAlign::Stretch),
            },
            0,
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
        ui.push_key_with_order(
            KEY_RESULTS_LIST,
            Box::new(ListView {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 190.0,
                },
                items: make_demo_items("", "Overview"),
                row_height: 28.0,
                scroll_offset: 0.0,
                key: "results_item",
                selected: Some(0),
                selection_anchor: Some(0),
                style: ListViewStyle::default(),
                focused: false,
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(190.0),
                align_self: Some(CrossAlign::Stretch),
            },
            1,
        );
        ui.push_key_with_order(
            KEY_CONTROLS_FIELD,
            Box::new(FormField {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 408.0,
                    height: 128.0,
                },
                label: "Controls".to_string(),
                helper_text: "Use Tab/Shift+Tab for focus traversal".to_string(),
                error_text: String::new(),
                has_error: false,
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
                    row.push_key_with_order(
                        KEY_CTRL_QUERY,
                        Box::new(TextInput {
                            key: "search_query",
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            value: String::new(),
                            placeholder: "Type here...",
                            style: TextInputStyle::default(),
                            focused: false,
                            cursor: 0,
                            selection_anchor: None,
                            dragging_selection: false,
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(2.0),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        1,
                    );
                    row.push_key_with_order(
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
                            width: SizeSpec::Flex(1.0),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        2,
                    );
                    row.push_key_with_order(
                        KEY_CTRL_MODAL_BTN,
                        Box::new(Button {
                            action: UiAction::OpenModal,
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            label: "Open Modal",
                            style: ButtonStyle {
                                idle_fill: "#1b2744",
                                hover_fill: "#2b3f6e",
                                pressed_fill: "#25365d",
                                border: "#4765a7",
                                focus_border: "#27ffd8",
                                text: REACTRON_THEME.text_primary,
                                font: "600 14px Consolas",
                            },
                            focused: false,
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(0.9),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        3,
                    );
                    row.push_key_with_order(
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
                            width: SizeSpec::Flex(0.9),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        4,
                    );
                    row.push_key_with_order(
                        KEY_CTRL_INTENSITY,
                        Box::new(Slider {
                            key: "ui_intensity",
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            value: 65.0,
                            min: 0.0,
                            max: 100.0,
                            step: 5.0,
                            label: "Intensity",
                            focused: false,
                            dragging: false,
                            style: SliderStyle::default(),
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(1.2),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        5,
                    );
                    row.push_key_with_order(
                        KEY_CTRL_ANIMATIONS,
                        Box::new(Checkbox {
                            key: "ui_animations",
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            value: true,
                            label: "Animations",
                            focused: false,
                            style: CheckboxStyle::default(),
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(0.9),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        6,
                    );
                    row.push_key_with_order(
                        KEY_CTRL_DENSITY,
                        Box::new(RadioGroup {
                            key: "ui_density",
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            label: "Density",
                            options: vec![
                                "Compact".to_string(),
                                "Cozy".to_string(),
                                "Comfort".to_string(),
                            ],
                            selected: 1,
                            focused: false,
                            style: RadioGroupStyle::default(),
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(1.3),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        7,
                    );
                    row.push_key_with_order(
                        KEY_CTRL_PRESET,
                        Box::new(Select {
                            key: "theme_preset",
                            rect: Rect {
                                x: 0.0,
                                y: 0.0,
                                width: 0.0,
                                height: 44.0,
                            },
                            options: vec![
                                "Ocean".to_string(),
                                "Sunset".to_string(),
                                "Forest".to_string(),
                            ],
                            selected: 0,
                            style: SelectStyle::default(),
                            focused: false,
                            open: false,
                            highlighted: 0,
                            label: "Preset",
                        }),
                        LayoutProps {
                            width: SizeSpec::Flex(0.95),
                            height: SizeSpec::Fixed(44.0),
                            align_self: Some(CrossAlign::Stretch),
                        },
                        8,
                    );
                    row
                },
                style: FormFieldStyle::default(),
                focused: false,
            }),
            LayoutProps {
                width: SizeSpec::Flex(1.0),
                height: SizeSpec::Fixed(128.0),
                align_self: Some(CrossAlign::Stretch),
            },
            2,
        );

        Self {
            state: DemoState {
                accent_on: true,
                neon_mode: true,
                query: String::new(),
                preset: "Ocean".to_string(),
                intensity: 65.0,
                animations: true,
                density: "Cozy".to_string(),
                active_tab: "Overview".to_string(),
                modal_result: "none".to_string(),
                show_modal: false,
                selected_item: "Widget Item 001".to_string(),
                ..DemoState::default()
            },
            ui,
            modal: Modal {
                key: "demo_modal_open",
                result_key: "demo_modal_result",
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                },
                open: false,
                title: "Reactron Modal".to_string(),
                message: "This is a reusable modal primitive with dismiss/cancel/confirm paths."
                    .to_string(),
                confirm_label: "Confirm",
                cancel_label: "Cancel",
                focused: true,
                style: ModalStyle::default(),
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
        let accent_primary = match self.state.preset.as_str() {
            "Sunset" => "#ff7849",
            "Forest" => "#43c06b",
            _ => REACTRON_THEME.accent_primary,
        };
        let accent_secondary = match self.state.preset.as_str() {
            "Sunset" => "#ffd166",
            "Forest" => "#9be564",
            _ => REACTRON_THEME.accent_secondary,
        };

        let triangle_color = if self.state.accent_on {
            accent_primary
        } else {
            accent_secondary
        };
        let hero_color = if self.state.neon_mode {
            if self.state.intensity >= 75.0 {
                triangle_color
            } else if self.state.intensity >= 40.0 {
                "#6ddfd0"
            } else {
                "#5c779f"
            }
        } else {
            "#7481a3"
        };

        let interaction_text = if self.state.pointer.is_down {
            "Pointer down: release on button to trigger"
        } else {
            "Tab/Shift+Tab focus | Left/Right on tabs | Esc closes modal"
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
            clicks_label.set_text(format!(
                "Clicks: {} | Tab: {} | Query: {} | Preset: {} | Intensity: {:.0}% | Animations: {} | Density: {} | Modal: {} | Selected: {}",
                self.state.clicks,
                self.state.active_tab,
                self.state.query,
                self.state.preset,
                self.state.intensity,
                if self.state.animations { "On" } else { "Off" },
                self.state.density,
                self.state.modal_result,
                self.state.selected_item
            ));
        }
        if let Some(hint_label) = self.ui.widget_mut_by_key::<Label>(KEY_HINT_LABEL) {
            hint_label.set_text(interaction_text.to_string());
        }
        if let Some(list) = self.ui.widget_mut_by_key::<ListView>(KEY_RESULTS_LIST) {
            list.set_items(make_demo_items(&self.state.query, &self.state.active_tab));
            list.set_selected_by_value(&self.state.selected_item);
        }
        if let Some(tabs) = self.ui.widget_mut_by_key::<Tabs>(KEY_TABS) {
            tabs.set_selected_by_value(&self.state.active_tab);
        }
        if let Some(field) = self.ui.widget_mut_by_key::<FormField>(KEY_CONTROLS_FIELD) {
            if let Some(input) = field.child_mut().widget_mut_by_key::<TextInput>(KEY_CTRL_QUERY) {
                input.set_value(self.state.query.clone());
            }
            if let Some(toggle) = field.child_mut().widget_mut_by_key::<Toggle>(KEY_CTRL_TOGGLE_NEON) {
                toggle.set_value(self.state.neon_mode);
            }
            if let Some(select) = field.child_mut().widget_mut_by_key::<Select>(KEY_CTRL_PRESET) {
                select.set_selected_by_value(&self.state.preset);
            }
            if let Some(slider) = field.child_mut().widget_mut_by_key::<Slider>(KEY_CTRL_INTENSITY) {
                slider.set_value(self.state.intensity);
            }
            if let Some(checkbox) = field.child_mut().widget_mut_by_key::<Checkbox>(KEY_CTRL_ANIMATIONS) {
                checkbox.set_value(self.state.animations);
            }
            if let Some(radio) = field.child_mut().widget_mut_by_key::<RadioGroup>(KEY_CTRL_DENSITY) {
                radio.set_selected_by_value(&self.state.density);
            }
            let validation = if self.state.query.trim().is_empty() {
                Some("Query is empty. Type to filter list items.".to_string())
            } else {
                None
            };
            field.set_validation(validation);
        }

        let mut ui_pointer = self.state.pointer.clone();
        if self.state.show_modal {
            ui_pointer.just_pressed = false;
            ui_pointer.just_released = false;
            ui_pointer.is_down = false;
            ui_pointer.suppress_focus_and_text_input();
            ui_pointer.scroll_y = 0.0;
        }

        let mut events = self.ui.draw(context, &ui_pointer);
        self.modal.set_open(self.state.show_modal);
        self.modal.set_rect(Rect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        });
        self.modal.set_message(format!(
            "Density: {} | Preset: {} | Intensity: {:.0}%",
            self.state.density, self.state.preset, self.state.intensity
        ));
        events.extend(self.modal.draw(context, &self.state.pointer));
        for event in events {
            match event {
                UiEvent::Action(UiAction::ToggleAccent) => {
                    self.state.accent_on = !self.state.accent_on;
                    self.state.clicks += 1;
                }
                UiEvent::Action(UiAction::SetNeon(enabled)) => {
                    self.state.neon_mode = enabled;
                }
                UiEvent::Action(UiAction::OpenModal) => {
                    self.state.show_modal = true;
                    self.state.modal_result = "opened".to_string();
                }
                UiEvent::ValueChanged {
                    key: "main_tab",
                    value,
                } => {
                    self.state.active_tab = value;
                    self.state.selected_item = String::new();
                }
                UiEvent::ValueChanged {
                    key: "search_query",
                    value,
                } => {
                    self.state.query = value;
                }
                UiEvent::ValueChanged {
                    key: "theme_preset",
                    value,
                } => {
                    self.state.preset = value;
                }
                UiEvent::ValueChanged {
                    key: "results_item",
                    value,
                } => {
                    self.state.selected_item = value;
                }
                UiEvent::ValueChanged {
                    key: "ui_intensity",
                    value,
                } => {
                    self.state.intensity = value.parse::<f64>().unwrap_or(self.state.intensity);
                }
                UiEvent::ValueChanged {
                    key: "ui_animations",
                    value,
                } => {
                    self.state.animations = value == "true";
                }
                UiEvent::ValueChanged {
                    key: "ui_density",
                    value,
                } => {
                    self.state.density = value;
                }
                UiEvent::ValueChanged {
                    key: "demo_modal_open",
                    value,
                } => {
                    self.state.show_modal = value == "true";
                }
                UiEvent::ValueChanged {
                    key: "demo_modal_result",
                    value,
                } => {
                    self.state.modal_result = value;
                }
                UiEvent::ValueChanged { .. } => {}
            }
        }

        self.state.pointer.reset_transient();
        Ok(())
    }
}

fn make_demo_items(query: &str, tab: &str) -> Vec<String> {
    let prefix = match tab {
        "Controls" => "Control",
        "Metrics" => "Metric",
        _ => "Widget",
    };
    let items = (1..=120)
        .map(|index| format!("{} Item {:03}", prefix, index))
        .collect::<Vec<_>>();

    if query.is_empty() {
        return items;
    }

    let query_lower = query.to_lowercase();
    items
        .into_iter()
        .filter(|item| item.to_lowercase().contains(&query_lower))
        .collect()
}
