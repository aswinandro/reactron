#[derive(Default, Clone)]
pub struct PointerState {
    pub x: f64,
    pub y: f64,
    pub is_down: bool,
    pub just_pressed: bool,
    pub just_released: bool,
    pub activate_primary: bool,
    pub focus_next: bool,
    pub text_input: Option<String>,
    pub backspace: bool,
    pub delete_forward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_left_select: bool,
    pub move_right_select: bool,
    pub move_word_left: bool,
    pub move_word_right: bool,
    pub move_word_left_select: bool,
    pub move_word_right_select: bool,
    pub move_up: bool,
    pub move_down: bool,
    pub move_page_up: bool,
    pub move_page_down: bool,
    pub move_home: bool,
    pub move_end: bool,
    pub select_all: bool,
    pub copy: bool,
    pub cut: bool,
    pub paste: bool,
    pub cancel: bool,
    pub scroll_y: f64,
}

pub enum PointerSignal {
    Move { x: f64, y: f64 },
    Down { x: f64, y: f64 },
    Up { x: f64, y: f64 },
    Leave,
    ActivatePrimary,
    FocusNext,
    TextInput(String),
    Backspace,
    DeleteForward,
    MoveLeft,
    MoveRight,
    MoveLeftSelect,
    MoveRightSelect,
    MoveWordLeft,
    MoveWordRight,
    MoveWordLeftSelect,
    MoveWordRightSelect,
    MoveUp,
    MoveDown,
    MovePageUp,
    MovePageDown,
    MoveHome,
    MoveEnd,
    SelectAll,
    Copy,
    Cut,
    Paste,
    Cancel,
    Scroll { x: f64, y: f64, delta_y: f64 },
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
            PointerSignal::ActivatePrimary => {
                self.activate_primary = true;
            }
            PointerSignal::FocusNext => {
                self.focus_next = true;
            }
            PointerSignal::TextInput(value) => {
                self.text_input = Some(value);
            }
            PointerSignal::Backspace => {
                self.backspace = true;
            }
            PointerSignal::DeleteForward => {
                self.delete_forward = true;
            }
            PointerSignal::MoveLeft => {
                self.move_left = true;
            }
            PointerSignal::MoveRight => {
                self.move_right = true;
            }
            PointerSignal::MoveLeftSelect => {
                self.move_left_select = true;
            }
            PointerSignal::MoveRightSelect => {
                self.move_right_select = true;
            }
            PointerSignal::MoveWordLeft => {
                self.move_word_left = true;
            }
            PointerSignal::MoveWordRight => {
                self.move_word_right = true;
            }
            PointerSignal::MoveWordLeftSelect => {
                self.move_word_left_select = true;
            }
            PointerSignal::MoveWordRightSelect => {
                self.move_word_right_select = true;
            }
            PointerSignal::MoveUp => {
                self.move_up = true;
            }
            PointerSignal::MoveDown => {
                self.move_down = true;
            }
            PointerSignal::MovePageUp => {
                self.move_page_up = true;
            }
            PointerSignal::MovePageDown => {
                self.move_page_down = true;
            }
            PointerSignal::MoveHome => {
                self.move_home = true;
            }
            PointerSignal::MoveEnd => {
                self.move_end = true;
            }
            PointerSignal::SelectAll => {
                self.select_all = true;
            }
            PointerSignal::Copy => {
                self.copy = true;
            }
            PointerSignal::Cut => {
                self.cut = true;
            }
            PointerSignal::Paste => {
                self.paste = true;
            }
            PointerSignal::Cancel => {
                self.cancel = true;
            }
            PointerSignal::Scroll { x, y, delta_y } => {
                self.x = x;
                self.y = y;
                self.scroll_y += delta_y;
            }
        }
    }

    pub fn reset_transient(&mut self) {
        self.just_pressed = false;
        self.just_released = false;
        self.activate_primary = false;
        self.focus_next = false;
        self.text_input = None;
        self.backspace = false;
        self.delete_forward = false;
        self.move_left = false;
        self.move_right = false;
        self.move_left_select = false;
        self.move_right_select = false;
        self.move_word_left = false;
        self.move_word_right = false;
        self.move_word_left_select = false;
        self.move_word_right_select = false;
        self.move_up = false;
        self.move_down = false;
        self.move_page_up = false;
        self.move_page_down = false;
        self.move_home = false;
        self.move_end = false;
        self.select_all = false;
        self.copy = false;
        self.cut = false;
        self.paste = false;
        self.cancel = false;
        self.scroll_y = 0.0;
    }
}
