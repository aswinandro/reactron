use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct TextInput {
    pub key: &'static str,
    pub rect: Rect,
    pub value: String,
    pub placeholder: &'static str,
    pub style: TextInputStyle,
    pub focused: bool,
    pub cursor: usize,
}

pub struct TextInputStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub text: &'static str,
    pub placeholder: &'static str,
    pub font: &'static str,
    pub padding_x: f64,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            fill: "#111827",
            border: "#2a3350",
            focus_border: "#27ffd8",
            text: "#d8e3ff",
            placeholder: "#6f7fa8",
            font: "600 16px Consolas",
            padding_x: 10.0,
        }
    }
}

impl TextInput {
    pub fn set_value(&mut self, value: String) {
        if self.value != value {
            self.value = value;
            self.cursor = self.value.len();
        } else if self.cursor > self.value.len() {
            self.cursor = self.value.len();
        }
    }

    fn prev_char_boundary(&self) -> usize {
        if self.cursor == 0 {
            0
        } else {
            self.value[..self.cursor]
                .char_indices()
                .last()
                .map(|(index, _)| index)
                .unwrap_or(0)
        }
    }

    fn next_char_boundary(&self) -> usize {
        if self.cursor >= self.value.len() {
            self.value.len()
        } else {
            self.value[self.cursor..]
                .char_indices()
                .nth(1)
                .map(|(offset, _)| self.cursor + offset)
                .unwrap_or(self.value.len())
        }
    }
}

impl Widget for TextInput {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        let mut events = Vec::new();

        if self.focused {
            if pointer.move_home {
                self.cursor = 0;
            }
            if pointer.move_end {
                self.cursor = self.value.len();
            }
            if pointer.move_left {
                self.cursor = self.prev_char_boundary();
            }
            if pointer.move_right {
                self.cursor = self.next_char_boundary();
            }
            if pointer.backspace {
                if self.cursor > 0 {
                    let start = self.prev_char_boundary();
                    self.value.replace_range(start..self.cursor, "");
                    self.cursor = start;
                    events.push(UiEvent::ValueChanged {
                        key: self.key,
                        value: self.value.clone(),
                    });
                }
            }
            if pointer.delete_forward {
                if self.cursor < self.value.len() {
                    let end = self.next_char_boundary();
                    self.value.replace_range(self.cursor..end, "");
                    events.push(UiEvent::ValueChanged {
                        key: self.key,
                        value: self.value.clone(),
                    });
                }
            }
            if let Some(input) = &pointer.text_input {
                self.value.insert_str(self.cursor, input);
                self.cursor += input.len();
                events.push(UiEvent::ValueChanged {
                    key: self.key,
                    value: self.value.clone(),
                });
            }
        }

        context.set_fill_style_str(self.style.fill);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        let border = if self.focused {
            self.style.focus_border
        } else {
            self.style.border
        };
        context.set_stroke_style_str(border);
        context.set_line_width(2.0);
        context.stroke_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        context.set_font(self.style.font);
        context.set_text_align("left");
        context.set_text_baseline("middle");

        let text_x = self.rect.x + self.style.padding_x;
        let text_y = self.rect.y + self.rect.height * 0.5;
        if self.value.is_empty() {
            context.set_fill_style_str(self.style.placeholder);
            let _ = context.fill_text(self.placeholder, text_x, text_y);
        } else {
            context.set_fill_style_str(self.style.text);
            let _ = context.fill_text(&self.value, text_x, text_y);
        }

        if self.focused {
            let before_cursor = &self.value[..self.cursor.min(self.value.len())];
            let width = context
                .measure_text(before_cursor)
                .ok()
                .map(|metrics| metrics.width())
                .unwrap_or(0.0);
            let cursor_x = text_x + width;
            let cursor_top = self.rect.y + 8.0;
            let cursor_height = (self.rect.height - 16.0).max(0.0);
            context.set_stroke_style_str(self.style.text);
            context.set_line_width(1.5);
            context.begin_path();
            context.move_to(cursor_x, cursor_top);
            context.line_to(cursor_x, cursor_top + cursor_height);
            context.stroke();
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
