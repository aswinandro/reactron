use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Label {
    pub rect: Rect,
    pub text: String,
    pub style: LabelStyle,
}

pub struct LabelStyle {
    pub font: &'static str,
    pub color: &'static str,
}

impl Label {
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Widget for Label {
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
        context.set_font(self.style.font);
        context.set_fill_style_str(self.style.color);
        context.set_text_align("center");
        context.set_text_baseline("middle");
        let _ = context.fill_text(
            &self.text,
            self.rect.x + self.rect.width / 2.0,
            self.rect.y + self.rect.height / 2.0,
        );
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
