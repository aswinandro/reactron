use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::core::navigation::{find_next_prefix, step_wrapped};
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Tabs {
    pub key: &'static str,
    pub rect: Rect,
    pub options: Vec<String>,
    pub selected: usize,
    pub focused: bool,
    pub style: TabsStyle,
}

pub struct TabsStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub active_fill: &'static str,
    pub text: &'static str,
    pub active_text: &'static str,
    pub font: &'static str,
}

impl Default for TabsStyle {
    fn default() -> Self {
        Self {
            fill: "#10192e",
            border: "#2a3350",
            focus_border: "#27ffd8",
            active_fill: "#22375e",
            text: "#9fb4e4",
            active_text: "#e5efff",
            font: "600 14px Consolas",
        }
    }
}

impl Tabs {
    pub fn set_selected_by_value(&mut self, value: &str) {
        if let Some(index) = self.options.iter().position(|tab| tab == value) {
            self.selected = index;
        }
    }

    fn selected_value(&self) -> String {
        self.options
            .get(self.selected)
            .cloned()
            .unwrap_or_else(String::new)
    }

    fn emit_changed(&self) -> UiEvent {
        UiEvent::ValueChanged {
            key: self.key,
            value: self.selected_value(),
        }
    }

    fn step(&mut self, delta: isize) -> Option<UiEvent> {
        if self.options.is_empty() {
            return None;
        }
        let next = step_wrapped(Some(self.selected), delta, self.options.len()).unwrap_or(0);
        if next != self.selected {
            self.selected = next;
            return Some(self.emit_changed());
        }
        None
    }

    fn jump_to(&mut self, text: &str) -> Option<UiEvent> {
        if let Some(index) = find_next_prefix(&self.options, text, Some(self.selected)) {
            if index != self.selected {
                self.selected = index;
                return Some(self.emit_changed());
            }
        }
        None
    }
}

impl Widget for Tabs {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        let mut events = Vec::new();
        if self.focused {
            if pointer.move_left || pointer.move_up {
                if let Some(event) = self.step(-1) {
                    events.push(event);
                }
            } else if pointer.move_right || pointer.move_down {
                if let Some(event) = self.step(1) {
                    events.push(event);
                }
            } else if pointer.move_home {
                if !self.options.is_empty() && self.selected != 0 {
                    self.selected = 0;
                    events.push(self.emit_changed());
                }
            } else if pointer.move_end && !self.options.is_empty() {
                let last = self.options.len() - 1;
                if self.selected != last {
                    self.selected = last;
                    events.push(self.emit_changed());
                }
            }
            if let Some(input) = &pointer.text_input {
                if let Some(event) = self.jump_to(input) {
                    events.push(event);
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

        let count = self.options.len().max(1) as f64;
        let tab_width = self.rect.width / count;
        context.set_font(self.style.font);
        context.set_text_baseline("middle");
        context.set_text_align("center");

        for (index, option) in self.options.iter().enumerate() {
            let x = self.rect.x + index as f64 * tab_width;
            let is_selected = self.selected == index;
            if is_selected {
                context.set_fill_style_str(self.style.active_fill);
                context.fill_rect(x, self.rect.y, tab_width, self.rect.height);
            }
            context.set_stroke_style_str(self.style.border);
            context.set_line_width(1.0);
            context.stroke_rect(x, self.rect.y, tab_width, self.rect.height);
            context.set_fill_style_str(if is_selected {
                self.style.active_text
            } else {
                self.style.text
            });
            let _ = context.fill_text(
                option,
                x + tab_width * 0.5,
                self.rect.y + self.rect.height * 0.5,
            );

            let hit = Rect {
                x,
                y: self.rect.y,
                width: tab_width,
                height: self.rect.height,
            };
            if hit.contains(pointer.x, pointer.y) && pointer.just_released && self.selected != index {
                self.selected = index;
                events.push(self.emit_changed());
            }
        }

        events
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
