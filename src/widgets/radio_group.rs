use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::core::navigation::step_wrapped;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct RadioGroup {
    pub key: &'static str,
    pub rect: Rect,
    pub label: &'static str,
    pub options: Vec<String>,
    pub selected: usize,
    pub focused: bool,
    pub style: RadioGroupStyle,
}

pub struct RadioGroupStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub selected_fill: &'static str,
    pub text: &'static str,
    pub font: &'static str,
}

impl Default for RadioGroupStyle {
    fn default() -> Self {
        Self {
            fill: "#111827",
            border: "#2a3350",
            focus_border: "#27ffd8",
            selected_fill: "#1f3b66",
            text: "#d8e3ff",
            font: "600 14px Consolas",
        }
    }
}

impl RadioGroup {
    pub fn set_selected_by_value(&mut self, value: &str) {
        if let Some(index) = self.options.iter().position(|option| option == value) {
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
}

impl Widget for RadioGroup {
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
        let _ = context.fill_text(self.label, self.rect.x + 8.0, self.rect.y + 11.0);

        let body_y = self.rect.y + 18.0;
        let body_height = (self.rect.height - 22.0).max(0.0);
        let option_count = self.options.len().max(1) as f64;
        let option_width = (self.rect.width - 8.0) / option_count;

        for (index, option) in self.options.iter().enumerate() {
            let x = self.rect.x + 4.0 + index as f64 * option_width;
            let w = option_width - 4.0;
            let option_rect = Rect {
                x,
                y: body_y,
                width: w,
                height: body_height,
            };
            let hovered = option_rect.contains(pointer.x, pointer.y);
            if hovered && pointer.just_released && self.selected != index {
                self.selected = index;
                events.push(self.emit_changed());
            }

            if self.selected == index {
                context.set_fill_style_str(self.style.selected_fill);
                context.fill_rect(option_rect.x, option_rect.y, option_rect.width, option_rect.height);
            }
            context.set_stroke_style_str(self.style.border);
            context.set_line_width(1.0);
            context.stroke_rect(option_rect.x, option_rect.y, option_rect.width, option_rect.height);

            context.set_fill_style_str(self.style.text);
            context.set_text_align("center");
            let _ = context.fill_text(
                option,
                option_rect.x + option_rect.width * 0.5,
                option_rect.y + option_rect.height * 0.5,
            );
        }

        context.set_text_align("left");
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
