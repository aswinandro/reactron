use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiAction, UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Toggle {
    pub rect: Rect,
    pub value: bool,
    pub label: &'static str,
    pub style: ToggleStyle,
    pub focused: bool,
}

pub struct ToggleStyle {
    pub on_fill: &'static str,
    pub off_fill: &'static str,
    pub knob_fill: &'static str,
    pub text_fill: &'static str,
    pub focus_border: &'static str,
    pub font: &'static str,
}

impl Default for ToggleStyle {
    fn default() -> Self {
        Self {
            on_fill: "#1f8f7c",
            off_fill: "#2d344a",
            knob_fill: "#d8e3ff",
            text_fill: "#d8e3ff",
            focus_border: "#27ffd8",
            font: "600 15px Consolas",
        }
    }
}

impl Toggle {
    pub fn set_value(&mut self, value: bool) {
        self.value = value;
    }
}

impl Widget for Toggle {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        let hovered = self.rect.contains(pointer.x, pointer.y);
        let clicked = hovered && pointer.just_released;
        if clicked {
            self.value = !self.value;
        }

        context.set_font(self.style.font);
        context.set_fill_style_str(self.style.text_fill);
        context.set_text_align("left");
        context.set_text_baseline("middle");
        let label_y = self.rect.y + self.rect.height / 2.0;
        let _ = context.fill_text(self.label, self.rect.x, label_y);

        let track_width = 68.0;
        let track_height = 30.0;
        let track_x = self.rect.x + self.rect.width - track_width;
        let track_y = self.rect.y + (self.rect.height - track_height) * 0.5;
        context.set_fill_style_str(if self.value {
            self.style.on_fill
        } else {
            self.style.off_fill
        });
        context.fill_rect(track_x, track_y, track_width, track_height);
        if self.focused {
            context.set_stroke_style_str(self.style.focus_border);
            context.set_line_width(2.0);
            context.stroke_rect(track_x, track_y, track_width, track_height);
        }

        let knob_size = 24.0;
        let knob_x = if self.value {
            track_x + track_width - knob_size - 3.0
        } else {
            track_x + 3.0
        };
        let knob_y = track_y + 3.0;
        context.set_fill_style_str(self.style.knob_fill);
        context.fill_rect(knob_x, knob_y, knob_size, knob_size);

        if clicked {
            vec![UiEvent::Action(UiAction::SetNeon(self.value))]
        } else {
            Vec::new()
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn activate(&mut self) -> Option<UiEvent> {
        self.value = !self.value;
        Some(UiEvent::Action(UiAction::SetNeon(self.value)))
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
