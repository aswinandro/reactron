use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::core::layout::VerticalLayout;
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub enum UiEvent {
    Action(UiAction),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UiAction {
    ToggleAccent,
    SetNeon(bool),
}

pub trait Widget {
    fn desired_size(&self) -> (f64, f64);
    fn set_rect(&mut self, rect: Rect);
    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Option<UiEvent>;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum LayoutDirection {
    Column,
    Row,
}

struct WidgetEntry {
    key: &'static str,
    widget: Box<dyn Widget>,
}

pub struct UiTree {
    widgets: Vec<WidgetEntry>,
    area: Rect,
    gap: f64,
    direction: LayoutDirection,
}

impl UiTree {
    pub fn column(area: Rect, gap: f64) -> Self {
        Self {
            widgets: Vec::new(),
            area,
            gap,
            direction: LayoutDirection::Column,
        }
    }

    #[allow(dead_code)]
    pub fn row(area: Rect, gap: f64) -> Self {
        Self {
            widgets: Vec::new(),
            area,
            gap,
            direction: LayoutDirection::Row,
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(WidgetEntry {
            key: "",
            widget,
        });
    }

    pub fn push_key(&mut self, key: &'static str, widget: Box<dyn Widget>) {
        self.widgets.push(WidgetEntry { key, widget });
    }

    pub fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    #[allow(dead_code)]
    pub fn widget_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.widgets
            .get_mut(index)
            .and_then(|entry| entry.widget.as_any_mut().downcast_mut::<T>())
    }

    pub fn widget_mut_by_key<T: 'static>(&mut self, key: &str) -> Option<&mut T> {
        self.widgets
            .iter_mut()
            .find(|entry| entry.key == key)
            .and_then(|entry| entry.widget.as_any_mut().downcast_mut::<T>())
    }

    pub fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> Vec<UiEvent> {
        if self.widgets.is_empty() {
            return Vec::new();
        }

        match self.direction {
            LayoutDirection::Column => self.draw_column(context, pointer),
            LayoutDirection::Row => self.draw_row(context, pointer),
        }
    }

    fn draw_column(
        &mut self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let mut layout = VerticalLayout::new(self.area.x, self.area.y, self.area.width, self.gap);

        for entry in &mut self.widgets {
            let (_, desired_height) = entry.widget.desired_size();
            let rect = layout.next(desired_height);
            entry.widget.set_rect(rect);
            if let Some(event) = entry.widget.draw(context, pointer) {
                events.push(event);
            }
        }

        events
    }

    fn draw_row(
        &mut self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let count = self.widgets.len();
        let total_gap = self.gap * (count.saturating_sub(1) as f64);

        let mut fixed_width = 0.0;
        let mut flexible = 0usize;
        for entry in &self.widgets {
            let (w, _) = entry.widget.desired_size();
            if w > 0.0 {
                fixed_width += w;
            } else {
                flexible += 1;
            }
        }

        let remaining = (self.area.width - fixed_width - total_gap).max(0.0);
        let flex_width = if flexible > 0 {
            remaining / flexible as f64
        } else {
            0.0
        };

        let mut x = self.area.x;
        for entry in &mut self.widgets {
            let (desired_w, desired_h) = entry.widget.desired_size();
            let width = if desired_w > 0.0 { desired_w } else { flex_width };
            let height = if desired_h > 0.0 {
                desired_h.min(self.area.height)
            } else {
                self.area.height
            };
            let y = self.area.y + (self.area.height - height) * 0.5;

            entry.widget.set_rect(Rect {
                x,
                y,
                width,
                height,
            });

            if let Some(event) = entry.widget.draw(context, pointer) {
                events.push(event);
            }

            x += width + self.gap;
        }

        events
    }
}
