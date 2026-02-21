use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::core::layout::VerticalLayout;
use web_sys::CanvasRenderingContext2d;

pub enum UiEvent {
    Action(&'static str),
}

pub trait Widget {
    fn desired_height(&self) -> f64;
    fn set_rect(&mut self, rect: Rect);
    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Option<UiEvent>;
}

pub struct UiTree {
    widgets: Vec<Box<dyn Widget>>,
    area: Rect,
    gap: f64,
}

impl UiTree {
    pub fn new(area: Rect, gap: f64) -> Self {
        Self {
            widgets: Vec::new(),
            area,
            gap,
        }
    }

    pub fn push(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    pub fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let mut layout = VerticalLayout::new(self.area.x, self.area.y, self.area.width, self.gap);

        for widget in &mut self.widgets {
            let rect = layout.next(widget.desired_height());
            widget.set_rect(rect);
            if let Some(event) = widget.draw(context, pointer) {
                events.push(event);
            }
        }

        events
    }
}
