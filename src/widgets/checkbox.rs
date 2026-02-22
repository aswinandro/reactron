use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Checkbox {
    pub key: &'static str,
    pub rect: Rect,
    pub value: bool,
    pub label: &'static str,
    pub focused: bool,
    pub style: CheckboxStyle,
}

pub struct CheckboxStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub check: &'static str,
    pub text: &'static str,
    pub font: &'static str,
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        Self {
            fill: "#0f172a",
            border: "#2a3350",
            focus_border: "#27ffd8",
            check: "#27ffd8",
            text: "#d8e3ff",
            font: "600 14px Consolas",
        }
    }
}

impl Checkbox {
    pub fn set_value(&mut self, value: bool) {
        self.value = value;
    }

    fn emit_changed(&self) -> UiEvent {
        UiEvent::ValueChanged {
            key: self.key,
            value: self.value.to_string(),
        }
    }
}

impl Widget for Checkbox {
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
            self.value = !self.value;
            events.push(self.emit_changed());
        } else if self.focused
            && (pointer.activate_primary || pointer.move_left || pointer.move_right)
        {
            self.value = !self.value;
            events.push(self.emit_changed());
        }

        context.set_font(self.style.font);
        context.set_fill_style_str(self.style.text);
        context.set_text_align("left");
        context.set_text_baseline("middle");

        let box_size = 18.0;
        let box_x = self.rect.x + 2.0;
        let box_y = self.rect.y + (self.rect.height - box_size) * 0.5;

        context.set_fill_style_str(self.style.fill);
        context.fill_rect(box_x, box_y, box_size, box_size);
        context.set_stroke_style_str(if self.focused {
            self.style.focus_border
        } else {
            self.style.border
        });
        context.set_line_width(2.0);
        context.stroke_rect(box_x, box_y, box_size, box_size);

        if self.value {
            context.set_stroke_style_str(self.style.check);
            context.set_line_width(2.5);
            context.begin_path();
            context.move_to(box_x + 4.0, box_y + 9.5);
            context.line_to(box_x + 8.0, box_y + 14.0);
            context.line_to(box_x + 14.0, box_y + 5.0);
            context.stroke();
        }

        let _ = context.fill_text(
            self.label,
            box_x + box_size + 8.0,
            self.rect.y + self.rect.height * 0.5,
        );

        events
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn activate(&mut self) -> Option<UiEvent> {
        self.value = !self.value;
        Some(self.emit_changed())
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
