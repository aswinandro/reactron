use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{EdgeInsets, UiEvent, UiTree, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Panel {
    pub rect: Rect,
    pub style: PanelStyle,
    pub padding: EdgeInsets,
    pub child: UiTree,
}

pub struct PanelStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub border_width: f64,
}

impl Panel {
    pub fn child_mut(&mut self) -> &mut UiTree {
        &mut self.child
    }
}

impl Widget for Panel {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        context.set_fill_style_str(self.style.fill);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        if self.style.border_width > 0.0 {
            context.set_stroke_style_str(self.style.border);
            context.set_line_width(self.style.border_width);
            context.stroke_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        }

        let child_area = Rect {
            x: self.rect.x + self.padding.left,
            y: self.rect.y + self.padding.top,
            width: (self.rect.width - self.padding.left - self.padding.right).max(0.0),
            height: (self.rect.height - self.padding.top - self.padding.bottom).max(0.0),
        };
        self.child.set_area(child_area);
        self.child.draw(context, pointer)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

