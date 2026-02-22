use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::core::navigation::{find_next_prefix, step_wrapped};
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Select {
    pub key: &'static str,
    pub rect: Rect,
    pub options: Vec<String>,
    pub selected: usize,
    pub style: SelectStyle,
    pub focused: bool,
    pub open: bool,
    pub highlighted: usize,
    pub label: &'static str,
}

pub struct SelectStyle {
    pub fill: &'static str,
    pub dropdown_fill: &'static str,
    pub option_hover_fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub text: &'static str,
    pub font: &'static str,
    pub option_height: f64,
}

impl Default for SelectStyle {
    fn default() -> Self {
        Self {
            fill: "#111827",
            dropdown_fill: "#0f172a",
            option_hover_fill: "#1f2937",
            border: "#2a3350",
            focus_border: "#27ffd8",
            text: "#d8e3ff",
            font: "600 15px Consolas",
            option_height: 34.0,
        }
    }
}

impl Select {
    pub fn set_selected_by_value(&mut self, value: &str) {
        if let Some(index) = self.options.iter().position(|option| option == value) {
            self.selected = index;
            self.highlighted = index;
        }
    }

    fn selected_value(&self) -> String {
        self.options
            .get(self.selected)
            .cloned()
            .unwrap_or_else(|| String::new())
    }

    fn step_next(&mut self) -> Option<UiEvent> {
        if self.options.is_empty() {
            return None;
        }
        self.selected = step_wrapped(Some(self.selected), 1, self.options.len()).unwrap_or(0);
        Some(UiEvent::ValueChanged {
            key: self.key,
            value: self.selected_value(),
        })
    }

    fn step_prev(&mut self) -> Option<UiEvent> {
        if self.options.is_empty() {
            return None;
        }
        self.selected = step_wrapped(Some(self.selected), -1, self.options.len()).unwrap_or(0);
        Some(UiEvent::ValueChanged {
            key: self.key,
            value: self.selected_value(),
        })
    }

    fn set_highlight_from_selected(&mut self) {
        self.highlighted = self.selected.min(self.options.len().saturating_sub(1));
    }

    fn dropdown_rect(&self) -> Rect {
        Rect {
            x: self.rect.x,
            y: self.rect.y + self.rect.height + 2.0,
            width: self.rect.width,
            height: self.options.len() as f64 * self.style.option_height,
        }
    }

    fn option_rect(&self, index: usize) -> Rect {
        Rect {
            x: self.rect.x,
            y: self.rect.y + self.rect.height + 2.0 + index as f64 * self.style.option_height,
            width: self.rect.width,
            height: self.style.option_height,
        }
    }

    fn jump_to_option(&mut self, needle: &str) -> Option<UiEvent> {
        if needle.is_empty() || self.options.is_empty() {
            return None;
        }

        if let Some(index) = find_next_prefix(&self.options, needle, Some(self.selected)) {
            self.selected = index;
            self.highlighted = index;
            return Some(UiEvent::ValueChanged {
                key: self.key,
                value: self.selected_value(),
            });
        }

        None
    }
}

impl Widget for Select {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let hovered = self.rect.contains(pointer.x, pointer.y);

        if self.focused && !self.open {
            if pointer.move_left {
                if let Some(event) = self.step_prev() {
                    events.push(event);
                }
            } else if pointer.move_right {
                if let Some(event) = self.step_next() {
                    events.push(event);
                }
            } else if pointer.move_up {
                if let Some(event) = self.step_prev() {
                    events.push(event);
                }
            } else if pointer.move_down {
                if let Some(event) = self.step_next() {
                    events.push(event);
                }
            }

            if let Some(input) = &pointer.text_input {
                if let Some(event) = self.jump_to_option(input) {
                    events.push(event);
                }
            }
        } else if self.focused && self.open {
            if pointer.move_up && !self.options.is_empty() {
                self.highlighted = if self.highlighted == 0 {
                    self.options.len() - 1
                } else {
                    self.highlighted - 1
                };
            } else if pointer.move_down && !self.options.is_empty() {
                self.highlighted = (self.highlighted + 1) % self.options.len();
            } else if pointer.cancel {
                self.open = false;
            }
        }

        if pointer.just_released {
            if hovered {
                self.open = !self.open;
                if self.open {
                    self.set_highlight_from_selected();
                }
            } else if self.open {
                let mut clicked_option = false;
                for (index, _) in self.options.iter().enumerate() {
                    let option_rect = self.option_rect(index);
                    if option_rect.contains(pointer.x, pointer.y) {
                        self.selected = index;
                        self.highlighted = index;
                        events.push(UiEvent::ValueChanged {
                            key: self.key,
                            value: self.selected_value(),
                        });
                        clicked_option = true;
                        break;
                    }
                }
                self.open = false;
                if !clicked_option {
                    let dropdown = self.dropdown_rect();
                    if !dropdown.contains(pointer.x, pointer.y) {
                        self.open = false;
                    }
                }
            }
        }

        context.set_fill_style_str(self.style.fill);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        context.set_stroke_style_str(if self.focused {
            self.style.focus_border
        } else {
            self.style.border
        });
        context.set_line_width(2.0);
        context.stroke_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        context.set_font(self.style.font);
        context.set_fill_style_str(self.style.text);
        context.set_text_align("left");
        context.set_text_baseline("middle");
        let arrow = if self.open { "v" } else { ">" };
        let text = format!("{}: {}  {}", self.label, self.selected_value(), arrow);
        let _ = context.fill_text(&text, self.rect.x + 10.0, self.rect.y + self.rect.height * 0.5);

        if self.open {
            let dropdown_rect = self.dropdown_rect();
            context.set_fill_style_str(self.style.dropdown_fill);
            context.fill_rect(
                dropdown_rect.x,
                dropdown_rect.y,
                dropdown_rect.width,
                dropdown_rect.height,
            );
            context.set_stroke_style_str(self.style.border);
            context.set_line_width(1.0);
            context.stroke_rect(
                dropdown_rect.x,
                dropdown_rect.y,
                dropdown_rect.width,
                dropdown_rect.height,
            );

            for (index, option) in self.options.iter().enumerate() {
                let option_rect = self.option_rect(index);
                let option_hovered = option_rect.contains(pointer.x, pointer.y);
                if option_hovered || index == self.highlighted {
                    context.set_fill_style_str(self.style.option_hover_fill);
                    context.fill_rect(
                        option_rect.x,
                        option_rect.y,
                        option_rect.width,
                        option_rect.height,
                    );
                }
                context.set_fill_style_str(self.style.text);
                let _ = context.fill_text(
                    option,
                    option_rect.x + 10.0,
                    option_rect.y + option_rect.height * 0.5,
                );
            }
        }

        events
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        if !focused {
            self.open = false;
        }
    }

    fn activate(&mut self) -> Option<UiEvent> {
        if self.open {
            if !self.options.is_empty() {
                self.selected = self.highlighted.min(self.options.len() - 1);
            }
            self.open = false;
            Some(UiEvent::ValueChanged {
                key: self.key,
                value: self.selected_value(),
            })
        } else {
            self.open = true;
            self.set_highlight_from_selected();
            None
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
