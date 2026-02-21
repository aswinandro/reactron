use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct ListView {
    pub rect: Rect,
    pub items: Vec<String>,
    pub row_height: f64,
    pub scroll_offset: f64,
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
        let hovered = self.rect.contains(pointer.x, pointer.y);
        if hovered && pointer.scroll_y.abs() > 0.0 {
            self.scroll_offset += pointer.scroll_y * 0.75;
        }

        let content_height = self.items.len() as f64 * self.row_height;
        let max_scroll = (content_height - self.rect.height).max(0.0);
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);

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

        for index in start_index..end_index {
            let y = self.rect.y + (index as f64 * self.row_height - self.scroll_offset);
            let row_color = if index % 2 == 0 {
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
        Vec::new()
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

