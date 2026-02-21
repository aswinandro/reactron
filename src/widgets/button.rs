use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiAction, UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Button {
    pub action: UiAction,
    pub rect: Rect,
    pub label: &'static str,
    pub style: ButtonStyle,
}

pub struct ButtonStyle {
    pub idle_fill: &'static str,
    pub hover_fill: &'static str,
    pub pressed_fill: &'static str,
    pub border: &'static str,
    pub text: &'static str,
    pub font: &'static str,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            idle_fill: "#18233d",
            hover_fill: "#283960",
            pressed_fill: "#1f2a47",
            border: "#3d5387",
            text: "#d8e3ff",
            font: "600 22px Consolas",
        }
    }
}

pub struct ButtonInteraction {
    pub clicked: bool,
}

impl Button {
    fn render_interaction(
        &self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> ButtonInteraction {
        let hovered = self.rect.contains(pointer.x, pointer.y);
        let pressed = hovered && pointer.is_down;
        let clicked = hovered && pointer.just_released;

        let fill = if pressed {
            self.style.pressed_fill
        } else if hovered {
            self.style.hover_fill
        } else {
            self.style.idle_fill
        };

        context.set_fill_style_str(fill);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        context.set_stroke_style_str(self.style.border);
        context.set_line_width(2.0);
        context.stroke_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        context.set_font(self.style.font);
        context.set_fill_style_str(self.style.text);
        context.set_text_align("center");
        context.set_text_baseline("middle");
        let _ = context.fill_text(
            self.label,
            self.rect.x + self.rect.width / 2.0,
            self.rect.y + self.rect.height / 2.0,
        );

        ButtonInteraction { clicked }
    }
}

impl Widget for Button {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Option<UiEvent> {
        let interaction = self.render_interaction(context, pointer);
        if interaction.clicked {
            Some(UiEvent::Action(self.action))
        } else {
            None
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
