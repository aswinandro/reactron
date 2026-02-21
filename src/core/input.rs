#[derive(Default, Clone, Copy)]
pub struct PointerState {
    pub x: f64,
    pub y: f64,
    pub is_down: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

pub enum PointerSignal {
    Move { x: f64, y: f64 },
    Down { x: f64, y: f64 },
    Up { x: f64, y: f64 },
    Leave,
}

impl PointerState {
    pub fn apply(&mut self, signal: PointerSignal) {
        match signal {
            PointerSignal::Move { x, y } => {
                self.x = x;
                self.y = y;
            }
            PointerSignal::Down { x, y } => {
                self.x = x;
                self.y = y;
                if !self.is_down {
                    self.is_down = true;
                    self.just_pressed = true;
                }
            }
            PointerSignal::Up { x, y } => {
                self.x = x;
                self.y = y;
                if self.is_down {
                    self.is_down = false;
                    self.just_released = true;
                }
            }
            PointerSignal::Leave => {
                if self.is_down {
                    self.is_down = false;
                    self.just_released = true;
                }
            }
        }
    }

    pub fn reset_transient(&mut self) {
        self.just_pressed = false;
        self.just_released = false;
    }
}

