use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Container {
    pub rect: Rect,
    pub style: ContainerStyle,
}

pub struct ContainerStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub border_width: f64,
}

impl Widget for Container {
    fn desired_height(&self) -> f64 {
        self.rect.height
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        _pointer: &PointerState,
    ) -> Option<UiEvent> {
        context.set_fill_style_str(self.style.fill);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        if self.style.border_width > 0.0 {
            context.set_stroke_style_str(self.style.border);
            context.set_line_width(self.style.border_width);
            context.stroke_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        }

        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
