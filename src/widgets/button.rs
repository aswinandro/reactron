use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use web_sys::CanvasRenderingContext2d;

pub struct Button {
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
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            idle_fill: "#18233d",
            hover_fill: "#283960",
            pressed_fill: "#1f2a47",
            border: "#3d5387",
            text: "#d8e3ff",
        }
    }
}

pub struct ButtonInteraction {
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

impl Button {
    pub fn draw(&self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> ButtonInteraction {
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

        context.set_font("600 22px Consolas");
        context.set_fill_style_str(self.style.text);
        context.set_text_align("center");
        context.set_text_baseline("middle");
        let _ = context.fill_text(
            self.label,
            self.rect.x + self.rect.width / 2.0,
            self.rect.y + self.rect.height / 2.0,
        );

        ButtonInteraction {
            hovered,
            pressed,
            clicked,
        }
    }
}

