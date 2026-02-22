use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Modal {
    pub key: &'static str,
    pub result_key: &'static str,
    pub rect: Rect,
    pub open: bool,
    pub title: String,
    pub message: String,
    pub confirm_label: &'static str,
    pub cancel_label: &'static str,
    pub focused: bool,
    pub style: ModalStyle,
}

pub struct ModalStyle {
    pub overlay: &'static str,
    pub panel_fill: &'static str,
    pub panel_border: &'static str,
    pub focus_border: &'static str,
    pub title: &'static str,
    pub text: &'static str,
    pub button_idle: &'static str,
    pub button_hover: &'static str,
    pub button_text: &'static str,
    pub title_font: &'static str,
    pub body_font: &'static str,
    pub button_font: &'static str,
}

impl Default for ModalStyle {
    fn default() -> Self {
        Self {
            overlay: "rgba(3,8,20,0.7)",
            panel_fill: "#0f172a",
            panel_border: "#2a3350",
            focus_border: "#27ffd8",
            title: "#e7eeff",
            text: "#b8c8ea",
            button_idle: "#1a2743",
            button_hover: "#283960",
            button_text: "#e2ebff",
            title_font: "700 18px Consolas",
            body_font: "15px Consolas",
            button_font: "600 14px Consolas",
        }
    }
}

impl Modal {
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn panel_rect(&self) -> Rect {
        let width = self.rect.width.min(460.0).max(280.0);
        let height = self.rect.height.min(220.0).max(170.0);
        Rect {
            x: self.rect.x + (self.rect.width - width) * 0.5,
            y: self.rect.y + (self.rect.height - height) * 0.5,
            width,
            height,
        }
    }

    fn confirm_rect(&self, panel: Rect) -> Rect {
        Rect {
            x: panel.x + panel.width - 220.0,
            y: panel.y + panel.height - 52.0,
            width: 96.0,
            height: 34.0,
        }
    }

    fn cancel_rect(&self, panel: Rect) -> Rect {
        Rect {
            x: panel.x + panel.width - 112.0,
            y: panel.y + panel.height - 52.0,
            width: 96.0,
            height: 34.0,
        }
    }

    fn emit_open(&self) -> UiEvent {
        UiEvent::ValueChanged {
            key: self.key,
            value: self.open.to_string(),
        }
    }

    fn emit_result(&self, result: &str) -> UiEvent {
        UiEvent::ValueChanged {
            key: self.result_key,
            value: result.to_string(),
        }
    }
}

impl Widget for Modal {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        if !self.open {
            return Vec::new();
        }

        let mut events = Vec::new();
        let panel = self.panel_rect();
        let confirm = self.confirm_rect(panel);
        let cancel = self.cancel_rect(panel);

        if pointer.cancel {
            self.open = false;
            events.push(self.emit_open());
            events.push(self.emit_result("cancel"));
        } else if pointer.just_released {
            let confirm_hovered = confirm.contains(pointer.x, pointer.y);
            let cancel_hovered = cancel.contains(pointer.x, pointer.y);
            let inside_panel = panel.contains(pointer.x, pointer.y);

            if confirm_hovered {
                self.open = false;
                events.push(self.emit_open());
                events.push(self.emit_result("confirm"));
            } else if cancel_hovered {
                self.open = false;
                events.push(self.emit_open());
                events.push(self.emit_result("cancel"));
            } else if !inside_panel {
                self.open = false;
                events.push(self.emit_open());
                events.push(self.emit_result("dismiss"));
            }
        }

        context.set_fill_style_str(self.style.overlay);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        context.set_fill_style_str(self.style.panel_fill);
        context.fill_rect(panel.x, panel.y, panel.width, panel.height);
        context.set_stroke_style_str(if self.focused {
            self.style.focus_border
        } else {
            self.style.panel_border
        });
        context.set_line_width(2.0);
        context.stroke_rect(panel.x, panel.y, panel.width, panel.height);

        context.set_fill_style_str(self.style.title);
        context.set_font(self.style.title_font);
        context.set_text_align("left");
        context.set_text_baseline("top");
        let _ = context.fill_text(&self.title, panel.x + 16.0, panel.y + 14.0);

        context.set_fill_style_str(self.style.text);
        context.set_font(self.style.body_font);
        let _ = context.fill_text(&self.message, panel.x + 16.0, panel.y + 52.0);

        let confirm_hovered = confirm.contains(pointer.x, pointer.y);
        let cancel_hovered = cancel.contains(pointer.x, pointer.y);
        for (button_rect, label, hovered) in [
            (confirm, self.confirm_label, confirm_hovered),
            (cancel, self.cancel_label, cancel_hovered),
        ] {
            context.set_fill_style_str(if hovered {
                self.style.button_hover
            } else {
                self.style.button_idle
            });
            context.fill_rect(
                button_rect.x,
                button_rect.y,
                button_rect.width,
                button_rect.height,
            );
            context.set_stroke_style_str(self.style.panel_border);
            context.set_line_width(1.0);
            context.stroke_rect(
                button_rect.x,
                button_rect.y,
                button_rect.width,
                button_rect.height,
            );

            context.set_fill_style_str(self.style.button_text);
            context.set_font(self.style.button_font);
            context.set_text_align("center");
            context.set_text_baseline("middle");
            let _ = context.fill_text(
                label,
                button_rect.x + button_rect.width * 0.5,
                button_rect.y + button_rect.height * 0.5,
            );
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
