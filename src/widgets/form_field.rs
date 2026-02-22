use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, UiTree, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct FormField {
    pub rect: Rect,
    pub label: String,
    pub helper_text: String,
    pub error_text: String,
    pub has_error: bool,
    pub child: UiTree,
    pub focused: bool,
    pub style: FormFieldStyle,
}

pub struct FormFieldStyle {
    pub label_font: &'static str,
    pub helper_font: &'static str,
    pub label_color: &'static str,
    pub helper_color: &'static str,
    pub error_color: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub field_fill: &'static str,
    pub label_height: f64,
    pub helper_height: f64,
    pub spacing: f64,
    pub padding: f64,
}

impl Default for FormFieldStyle {
    fn default() -> Self {
        Self {
            label_font: "600 14px Consolas",
            helper_font: "13px Consolas",
            label_color: "#d8e3ff",
            helper_color: "#91a4d6",
            error_color: "#ff7a7a",
            border: "#2a3350",
            focus_border: "#27ffd8",
            field_fill: "#0d1324",
            label_height: 18.0,
            helper_height: 16.0,
            spacing: 6.0,
            padding: 10.0,
        }
    }
}

impl FormField {
    pub fn child_mut(&mut self) -> &mut UiTree {
        &mut self.child
    }

    pub fn set_validation(&mut self, error_text: Option<String>) {
        match error_text {
            Some(text) => {
                self.has_error = true;
                self.error_text = text;
            }
            None => {
                self.has_error = false;
                self.error_text.clear();
            }
        }
    }
}

impl Widget for FormField {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        let top = self.rect.y;
        let label_y = top + self.style.label_height * 0.5;

        context.set_font(self.style.label_font);
        context.set_fill_style_str(self.style.label_color);
        context.set_text_align("left");
        context.set_text_baseline("middle");
        let _ = context.fill_text(&self.label, self.rect.x, label_y);

        let field_top = top + self.style.label_height + self.style.spacing;
        let helper_top = self.rect.y + self.rect.height - self.style.helper_height;
        let field_height = (helper_top - self.style.spacing - field_top).max(0.0);

        context.set_fill_style_str(self.style.field_fill);
        context.fill_rect(self.rect.x, field_top, self.rect.width, field_height);
        context.set_stroke_style_str(if self.focused {
            self.style.focus_border
        } else {
            self.style.border
        });
        context.set_line_width(1.5);
        context.stroke_rect(self.rect.x, field_top, self.rect.width, field_height);

        let child_area = Rect {
            x: self.rect.x + self.style.padding,
            y: field_top + self.style.padding,
            width: (self.rect.width - self.style.padding * 2.0).max(0.0),
            height: (field_height - self.style.padding * 2.0).max(0.0),
        };
        self.child.set_area(child_area);

        let mut child_pointer = pointer.clone();
        if !self.focused {
            child_pointer.suppress_focus_and_text_input();
        }
        let events = self.child.draw(context, &child_pointer);

        context.set_font(self.style.helper_font);
        context.set_fill_style_str(if self.has_error {
            self.style.error_color
        } else {
            self.style.helper_color
        });
        context.set_text_align("left");
        context.set_text_baseline("middle");
        let message = if self.has_error {
            &self.error_text
        } else {
            &self.helper_text
        };
        let _ = context.fill_text(
            message,
            self.rect.x,
            helper_top + self.style.helper_height * 0.5,
        );

        events
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn focus_next_in_children(&mut self) -> bool {
        self.child.focus_next()
    }

    fn focus_prev_in_children(&mut self) -> bool {
        self.child.focus_prev()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
