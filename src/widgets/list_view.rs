use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::core::navigation::{find_next_contains, step_clamped};
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct ListView {
    pub key: &'static str,
    pub rect: Rect,
    pub items: Vec<String>,
    pub row_height: f64,
    pub scroll_offset: f64,
    pub selected: Option<usize>,
    pub selection_anchor: Option<usize>,
    pub style: ListViewStyle,
    pub focused: bool,
}

pub struct ListViewStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub row_even: &'static str,
    pub row_odd: &'static str,
    pub text: &'static str,
    pub font: &'static str,
}

impl Default for ListViewStyle {
    fn default() -> Self {
        Self {
            fill: "#0f162a",
            border: "#2a3350",
            focus_border: "#27ffd8",
            row_even: "#141d34",
            row_odd: "#10182d",
            text: "#cfe0ff",
            font: "14px Consolas",
        }
    }
}

impl ListView {
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        if let Some(selected) = self.selected {
            if selected >= self.items.len() {
                self.selected = self.items.len().checked_sub(1);
            }
        }
    }

    pub fn set_selected_by_value(&mut self, value: &str) {
        self.selected = self.items.iter().position(|item| item == value);
        self.selection_anchor = self.selected;
    }

    fn selected_value(&self) -> String {
        self.selected
            .and_then(|index| self.items.get(index))
            .cloned()
            .unwrap_or_default()
    }

    fn emit_selection(&self) -> Option<UiEvent> {
        self.selected.map(|_| UiEvent::ValueChanged {
            key: self.key,
            value: self.selected_value(),
        })
    }

    fn max_scroll(&self) -> f64 {
        let content_height = self.items.len() as f64 * self.row_height;
        (content_height - self.rect.height).max(0.0)
    }

    fn ensure_selected_visible(&mut self) {
        if let Some(index) = self.selected {
            let top = index as f64 * self.row_height;
            let bottom = top + self.row_height;
            if top < self.scroll_offset {
                self.scroll_offset = top;
            } else if bottom > self.scroll_offset + self.rect.height {
                self.scroll_offset = (bottom - self.rect.height).max(0.0);
            }
            self.scroll_offset = self.scroll_offset.clamp(0.0, self.max_scroll());
        }
    }

    fn select_by_index(&mut self, index: usize) -> Option<UiEvent> {
        if self.items.is_empty() {
            self.selected = None;
            self.selection_anchor = None;
            return None;
        }
        let next = index.min(self.items.len() - 1);
        if self.selected != Some(next) {
            self.selected = Some(next);
            self.selection_anchor = Some(next);
            self.ensure_selected_visible();
            return self.emit_selection();
        }
        self.ensure_selected_visible();
        None
    }

    fn step_selection(&mut self, delta: isize) -> Option<UiEvent> {
        if self.items.is_empty() {
            self.selected = None;
            return None;
        }

        if let Some(next) = step_clamped(self.selected, delta, self.items.len()) {
            return self.select_by_index(next);
        }

        None
    }

    fn page_delta(&self) -> isize {
        (self.rect.height / self.row_height).floor().max(1.0) as isize
    }

    fn extend_selection(&mut self, delta: isize) -> Option<UiEvent> {
        if self.items.is_empty() {
            return None;
        }
        let Some(next) = step_clamped(self.selected, delta, self.items.len()) else {
            return None;
        };
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.selected.unwrap_or(next));
        }
        if self.selected != Some(next) {
            self.selected = Some(next);
            self.ensure_selected_visible();
            return self.emit_selection();
        }
        self.ensure_selected_visible();
        None
    }

    fn selection_range(&self) -> Option<(usize, usize)> {
        match (self.selection_anchor, self.selected) {
            (Some(a), Some(b)) => {
                if a <= b {
                    Some((a, b))
                } else {
                    Some((b, a))
                }
            }
            _ => None,
        }
    }

    fn jump_to_match(&mut self, needle: &str) -> Option<UiEvent> {
        if needle.is_empty() || self.items.is_empty() {
            return None;
        }

        if let Some(index) = find_next_contains(&self.items, needle, self.selected) {
            return self.select_by_index(index);
        }

        None
    }
}

impl Widget for ListView {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let hovered = self.rect.contains(pointer.x, pointer.y);
        if hovered && pointer.scroll_y.abs() > 0.0 {
            self.scroll_offset += pointer.scroll_y * 0.75;
        }

        if pointer.just_released && hovered && self.row_height > 0.0 {
            let y_in_list = (pointer.y - self.rect.y + self.scroll_offset).max(0.0);
            let index = (y_in_list / self.row_height).floor() as usize;
            if index < self.items.len() {
                if let Some(event) = self.select_by_index(index) {
                    events.push(event);
                }
            }
        }

        if self.focused {
            if pointer.move_up {
                self.selection_anchor = self.selected;
                if let Some(event) = self.step_selection(-1) {
                    events.push(event);
                }
            } else if pointer.move_down {
                self.selection_anchor = self.selected;
                if let Some(event) = self.step_selection(1) {
                    events.push(event);
                }
            } else if pointer.move_up_select {
                if let Some(event) = self.extend_selection(-1) {
                    events.push(event);
                }
            } else if pointer.move_down_select {
                if let Some(event) = self.extend_selection(1) {
                    events.push(event);
                }
            } else if pointer.move_page_up {
                self.selection_anchor = self.selected;
                if let Some(event) = self.step_selection(-self.page_delta()) {
                    events.push(event);
                }
            } else if pointer.move_page_down {
                self.selection_anchor = self.selected;
                if let Some(event) = self.step_selection(self.page_delta()) {
                    events.push(event);
                }
            } else if pointer.move_home {
                self.selection_anchor = self.selected;
                if let Some(event) = self.select_by_index(0) {
                    events.push(event);
                }
            } else if pointer.move_end && !self.items.is_empty() {
                self.selection_anchor = self.selected;
                if let Some(event) = self.select_by_index(self.items.len() - 1) {
                    events.push(event);
                }
            }
            if let Some(input) = &pointer.text_input {
                if let Some(event) = self.jump_to_match(input) {
                    events.push(event);
                }
            }
        }

        self.scroll_offset = self.scroll_offset.clamp(0.0, self.max_scroll());

        context.set_fill_style_str(self.style.fill);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        context.set_stroke_style_str(if self.focused {
            self.style.focus_border
        } else {
            self.style.border
        });
        context.set_line_width(2.0);
        context.stroke_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        context.save();
        context.begin_path();
        context.rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        context.clip();

        context.set_font(self.style.font);
        context.set_fill_style_str(self.style.text);
        context.set_text_align("left");
        context.set_text_baseline("middle");

        let start_index = (self.scroll_offset / self.row_height).floor().max(0.0) as usize;
        let visible_rows = (self.rect.height / self.row_height).ceil().max(0.0) as usize + 1;
        let end_index = (start_index + visible_rows).min(self.items.len());
        let selected_range = self.selection_range();

        for index in start_index..end_index {
            let y = self.rect.y + (index as f64 * self.row_height - self.scroll_offset);
            let in_range = selected_range
                .map(|(start, end)| index >= start && index <= end)
                .unwrap_or(false);
            let row_color = if in_range {
                "#274060"
            } else if index % 2 == 0 {
                self.style.row_even
            } else {
                self.style.row_odd
            };
            context.set_fill_style_str(row_color);
            context.fill_rect(self.rect.x, y, self.rect.width, self.row_height);

            context.set_fill_style_str(self.style.text);
            let _ = context.fill_text(&self.items[index], self.rect.x + 10.0, y + self.row_height * 0.5);
        }

        context.restore();
        events
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn activate(&mut self) -> Option<UiEvent> {
        self.emit_selection()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
