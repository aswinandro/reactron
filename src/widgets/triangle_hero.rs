use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct TriangleHero {
    pub rect: Rect,
    pub color: &'static str,
}

impl TriangleHero {
    pub fn set_color(&mut self, color: &'static str) {
        self.color = color;
    }
}

impl Widget for TriangleHero {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        _pointer: &PointerState,
    ) -> Option<UiEvent> {
        let cx = self.rect.x + self.rect.width / 2.0;
        let top = self.rect.y;
        let base_y = self.rect.y + self.rect.height;
        let half_base = self.rect.width * 0.32;

        context.begin_path();
        context.move_to(cx, top);
        context.line_to(cx - half_base, base_y);
        context.line_to(cx + half_base, base_y);
        context.close_path();
        context.set_fill_style_str(self.color);
        context.fill();
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
