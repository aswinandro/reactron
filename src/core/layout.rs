use crate::core::geometry::Rect;

pub struct VerticalLayout {
    x: f64,
    y: f64,
    width: f64,
    gap: f64,
}

impl VerticalLayout {
    pub fn new(x: f64, y: f64, width: f64, gap: f64) -> Self {
        Self { x, y, width, gap }
    }

    pub fn next(&mut self, height: f64) -> Rect {
        let rect = Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height,
        };
        self.y += height + self.gap;
        rect
    }
}

