use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use crate::ui::tree::{UiEvent, Widget};
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub struct Slider {
    pub key: &'static str,
    pub rect: Rect,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub label: &'static str,
    pub focused: bool,
    pub dragging: bool,
    pub style: SliderStyle,
}

pub struct SliderStyle {
    pub fill: &'static str,
    pub border: &'static str,
    pub focus_border: &'static str,
    pub text: &'static str,
    pub track_bg: &'static str,
    pub track_fill: &'static str,
    pub knob_fill: &'static str,
    pub font: &'static str,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self {
            fill: "#111827",
            border: "#2a3350",
            focus_border: "#27ffd8",
            text: "#d8e3ff",
            track_bg: "#1c2742",
            track_fill: "#27ffd8",
            knob_fill: "#f8fafc",
            font: "600 14px Consolas",
        }
    }
}

impl Slider {
    pub fn set_value(&mut self, value: f64) {
        self.value = self.clamp_and_snap(value);
    }

    fn clamp_and_snap(&self, value: f64) -> f64 {
        let range_min = self.min.min(self.max);
        let range_max = self.max.max(self.min);
        let step = self.step.abs().max(0.000_001);
        let snapped = ((value - range_min) / step).round() * step + range_min;
        snapped.clamp(range_min, range_max)
    }

    fn ratio(&self) -> f64 {
        let span = (self.max - self.min).abs();
        if span <= 0.000_001 {
            0.0
        } else {
            ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
        }
    }

    fn track_bounds(&self) -> (f64, f64) {
        let start = self.rect.x + 10.0;
        let end = self.rect.x + self.rect.width - 10.0;
        (start, end.max(start))
    }

    fn value_from_pointer_x(&self, x: f64) -> f64 {
        let (start, end) = self.track_bounds();
        let track_width = (end - start).max(0.000_001);
        let ratio = ((x - start) / track_width).clamp(0.0, 1.0);
        self.min + (self.max - self.min) * ratio
    }

    fn emit_changed(&self) -> UiEvent {
        UiEvent::ValueChanged {
            key: self.key,
            value: format!("{:.2}", self.value),
        }
    }

    fn apply_delta(&mut self, delta: f64) -> Option<UiEvent> {
        let next = self.clamp_and_snap(self.value + delta);
        if (next - self.value).abs() > 0.000_001 {
            self.value = next;
            return Some(self.emit_changed());
        }
        None
    }

    fn apply_pointer_value(&mut self, x: f64) -> Option<UiEvent> {
        let next = self.clamp_and_snap(self.value_from_pointer_x(x));
        if (next - self.value).abs() > 0.000_001 {
            self.value = next;
            return Some(self.emit_changed());
        }
        None
    }
}

impl Widget for Slider {
    fn desired_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent> {
        let mut events = Vec::new();

        if self.focused {
            if pointer.move_left || pointer.move_down {
                if let Some(event) = self.apply_delta(-self.step.abs()) {
                    events.push(event);
                }
            } else if pointer.move_right || pointer.move_up {
                if let Some(event) = self.apply_delta(self.step.abs()) {
                    events.push(event);
                }
            } else if pointer.move_page_down {
                if let Some(event) = self.apply_delta(-(self.step.abs() * 10.0)) {
                    events.push(event);
                }
            } else if pointer.move_page_up {
                if let Some(event) = self.apply_delta(self.step.abs() * 10.0) {
                    events.push(event);
                }
            } else if pointer.move_home {
                let next = self.clamp_and_snap(self.min);
                if (next - self.value).abs() > 0.000_001 {
                    self.value = next;
                    events.push(self.emit_changed());
                }
            } else if pointer.move_end {
                let next = self.clamp_and_snap(self.max);
                if (next - self.value).abs() > 0.000_001 {
                    self.value = next;
                    events.push(self.emit_changed());
                }
            }
        }

        if pointer.just_pressed && self.rect.contains(pointer.x, pointer.y) {
            self.dragging = true;
            if let Some(event) = self.apply_pointer_value(pointer.x) {
                events.push(event);
            }
        }
        if self.dragging && pointer.is_down {
            if let Some(event) = self.apply_pointer_value(pointer.x) {
                events.push(event);
            }
        }
        if pointer.just_released || pointer.cancel {
            self.dragging = false;
        }

        context.set_fill_style_str(self.style.fill);
        context.fill_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        context.set_stroke_style_str(if self.focused {
            self.style.focus_border
        } else {
            self.style.border
        });
        context.set_line_width(2.0);
        context.stroke_rect(self.rect.x, self.rect.y, self.rect.width, self.rect.height);

        context.set_font(self.style.font);
        context.set_fill_style_str(self.style.text);
        context.set_text_align("left");
        context.set_text_baseline("middle");
        let _ = context.fill_text(
            &format!("{}: {:.0}", self.label, self.value),
            self.rect.x + 10.0,
            self.rect.y + 12.0,
        );

        let (track_start, track_end) = self.track_bounds();
        let track_y = self.rect.y + self.rect.height - 14.0;
        let track_height = 6.0;
        let width = (track_end - track_start).max(0.0);
        let active_width = width * self.ratio();

        context.set_fill_style_str(self.style.track_bg);
        context.fill_rect(track_start, track_y, width, track_height);
        context.set_fill_style_str(self.style.track_fill);
        context.fill_rect(track_start, track_y, active_width, track_height);

        let knob_x = track_start + active_width;
        context.begin_path();
        let _ = context.arc(knob_x, track_y + track_height * 0.5, 6.0, 0.0, std::f64::consts::PI * 2.0);
        context.set_fill_style_str(self.style.knob_fill);
        context.fill();

        events
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        if !focused {
            self.dragging = false;
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
