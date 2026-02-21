use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use std::cell::RefCell;
use web_sys::CanvasRenderingContext2d;

thread_local! {
    static INTERNAL_CLIPBOARD: RefCell<String> = const { RefCell::new(String::new()) };
}

pub struct TextInput {
    pub key: &'static str,
    pub rect: Rect,
    pub value: String,
    pub placeholder: &'static str,
    pub style: TextInputStyle,
    pub focused: bool,
    pub cursor: usize,
    pub selection_anchor: Option<usize>,
    pub dragging_selection: bool,
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

    fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection_anchor.and_then(|anchor| {
            if anchor == self.cursor {
                None
            } else if anchor < self.cursor {
                Some((anchor, self.cursor))
            } else {
                Some((self.cursor, anchor))
            }
        })
    }

    fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    fn delete_selection_if_any(&mut self) -> bool {
        if let Some((start, end)) = self.selection_range() {
            self.value.replace_range(start..end, "");
            self.cursor = start;
            self.clear_selection();
            true
        } else {
            false
        }
    }

    fn set_cursor_from_x(&mut self, context: &CanvasRenderingContext2d, x: f64) {
        let mut best = 0usize;
        let mut best_distance = f64::MAX;
        for index in self.value.char_indices().map(|(i, _)| i).chain(std::iter::once(self.value.len())) {
            let width = context
                .measure_text(&self.value[..index])
                .ok()
                .map(|metrics| metrics.width())
                .unwrap_or(0.0);
            let distance = (width - x).abs();
            if distance < best_distance {
                best_distance = distance;
                best = index;
            }
        }
        self.cursor = best;
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
            if pointer.select_all {
                self.selection_anchor = Some(0);
                self.cursor = self.value.len();
            }

            if pointer.just_pressed && self.rect.contains(pointer.x, pointer.y) {
                context.set_font(self.style.font);
                let text_x = self.rect.x + self.style.padding_x;
                self.set_cursor_from_x(context, (pointer.x - text_x).max(0.0));
                self.selection_anchor = Some(self.cursor);
                self.dragging_selection = true;
            }
            if pointer.is_down && self.dragging_selection {
                context.set_font(self.style.font);
                let text_x = self.rect.x + self.style.padding_x;
                self.set_cursor_from_x(context, (pointer.x - text_x).max(0.0));
            }
            if pointer.just_released && self.dragging_selection {
                self.dragging_selection = false;
                if self.selection_range().is_none() {
                    self.clear_selection();
                }
            }

            if pointer.move_home {
                self.cursor = 0;
                self.clear_selection();
            }
            if pointer.move_end {
                self.cursor = self.value.len();
                self.clear_selection();
            }
            if pointer.move_left_select {
                if self.selection_anchor.is_none() {
                    self.selection_anchor = Some(self.cursor);
                }
                self.cursor = self.prev_char_boundary();
            }
            if pointer.move_right_select {
                if self.selection_anchor.is_none() {
                    self.selection_anchor = Some(self.cursor);
                }
                self.cursor = self.next_char_boundary();
            }
            if pointer.move_left {
                self.cursor = self.prev_char_boundary();
                self.clear_selection();
            }
            if pointer.move_right {
                self.cursor = self.next_char_boundary();
                self.clear_selection();
            }
            if pointer.backspace {
                if self.delete_selection_if_any() {
                    events.push(UiEvent::ValueChanged {
                        key: self.key,
                        value: self.value.clone(),
                    });
                } else if self.cursor > 0 {
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
                if self.delete_selection_if_any() {
                    events.push(UiEvent::ValueChanged {
                        key: self.key,
                        value: self.value.clone(),
                    });
                } else if self.cursor < self.value.len() {
                    let end = self.next_char_boundary();
                    self.value.replace_range(self.cursor..end, "");
                    events.push(UiEvent::ValueChanged {
                        key: self.key,
                        value: self.value.clone(),
                    });
                }
            }
            if let Some(input) = &pointer.text_input {
                let _ = self.delete_selection_if_any();
                self.value.insert_str(self.cursor, input);
                self.cursor += input.len();
                self.clear_selection();
                events.push(UiEvent::ValueChanged {
                    key: self.key,
                    value: self.value.clone(),
                });
            }
            if pointer.copy {
                if let Some((start, end)) = self.selection_range() {
                    let copied = self.value[start..end].to_string();
                    INTERNAL_CLIPBOARD.with(|buffer| {
                        *buffer.borrow_mut() = copied;
                    });
                }
            }
            if pointer.cut {
                if let Some((start, end)) = self.selection_range() {
                    let cut = self.value[start..end].to_string();
                    INTERNAL_CLIPBOARD.with(|buffer| {
                        *buffer.borrow_mut() = cut;
                    });
                    self.value.replace_range(start..end, "");
                    self.cursor = start;
                    self.clear_selection();
                    events.push(UiEvent::ValueChanged {
                        key: self.key,
                        value: self.value.clone(),
                    });
                }
            }
            if pointer.paste {
                let pasted = INTERNAL_CLIPBOARD.with(|buffer| buffer.borrow().clone());
                if !pasted.is_empty() {
                    let _ = self.delete_selection_if_any();
                    self.value.insert_str(self.cursor, &pasted);
                    self.cursor += pasted.len();
                    self.clear_selection();
                    events.push(UiEvent::ValueChanged {
                        key: self.key,
                        value: self.value.clone(),
                    });
                }
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
            if let Some((start, end)) = self.selection_range() {
                let start_width = context
                    .measure_text(&self.value[..start])
                    .ok()
                    .map(|metrics| metrics.width())
                    .unwrap_or(0.0);
                let end_width = context
                    .measure_text(&self.value[..end])
                    .ok()
                    .map(|metrics| metrics.width())
                    .unwrap_or(start_width);
                context.set_fill_style_str("rgba(39,255,216,0.28)");
                context.fill_rect(
                    text_x + start_width,
                    self.rect.y + 7.0,
                    (end_width - start_width).max(0.0),
                    (self.rect.height - 14.0).max(0.0),
                );
                context.set_fill_style_str(self.style.text);
                let _ = context.fill_text(&self.value, text_x, text_y);
            }

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
        if !focused {
            self.clear_selection();
            self.dragging_selection = false;
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
