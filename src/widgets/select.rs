use crate::core::geometry::Rect;
use crate::core::input::PointerState;
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
    pub label: &'static str,
}

pub struct SelectStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub text: &'static str,
    pub font: &'static str,
}

impl Default for SelectStyle {
    fn default() -> Self {
        Self {
            fill: "#111827",
            border: "#2a3350",
            focus_border: "#27ffd8",
            text: "#d8e3ff",
            font: "600 15px Consolas",
        }
    }
}

impl Select {
    pub fn set_selected_by_value(&mut self, value: &str) {
        if let Some(index) = self.options.iter().position(|option| option == value) {
            self.selected = index;
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
        self.selected = (self.selected + 1) % self.options.len();
        Some(UiEvent::ValueChanged {
            key: self.key,
            value: self.selected_value(),
        })
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
        if hovered && pointer.just_released {
            if let Some(event) = self.step_next() {
                events.push(event);
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
        let text = format!("{}: {}  >", self.label, self.selected_value());
        let _ = context.fill_text(&text, self.rect.x + 10.0, self.rect.y + self.rect.height * 0.5);

        events
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn activate(&mut self) -> Option<UiEvent> {
        self.step_next()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

