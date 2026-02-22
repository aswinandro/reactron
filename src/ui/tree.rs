use crate::core::geometry::Rect;
use crate::core::input::PointerState;
use std::any::Any;
use web_sys::CanvasRenderingContext2d;

pub enum UiEvent {
    Action(UiAction),
    ValueChanged {
        key: &'static str,
        value: String,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UiAction {
    ToggleAccent,
    SetNeon(bool),
    OpenModal,
}

pub trait Widget {
    fn desired_size(&self) -> (f64, f64);
    fn set_rect(&mut self, rect: Rect);
    fn draw(&mut self, context: &CanvasRenderingContext2d, pointer: &PointerState) -> Vec<UiEvent>;
    fn focusable(&self) -> bool {
        false
    }
    fn set_focused(&mut self, _focused: bool) {}
    fn activate(&mut self) -> Option<UiEvent> {
        None
    }
    fn focus_next_in_children(&mut self) -> bool {
        false
    }
    fn focus_prev_in_children(&mut self) -> bool {
        false
    }
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum LayoutDirection {
    Column,
    Row,
    Stack,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum CrossAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Copy)]
pub enum SizeSpec {
    Auto,
    Fixed(f64),
    Flex(f64),
}

#[derive(Clone, Copy)]
pub struct LayoutProps {
    pub width: SizeSpec,
    pub height: SizeSpec,
    pub align_self: Option<CrossAlign>,
}

impl LayoutProps {
    pub fn auto() -> Self {
        Self {
            width: SizeSpec::Auto,
            height: SizeSpec::Auto,
            align_self: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct EdgeInsets {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

impl EdgeInsets {
    pub fn all(value: f64) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }
}

struct WidgetEntry {
    key: &'static str,
    widget: Box<dyn Widget>,
    layout: LayoutProps,
    focus_order: i32,
}

pub struct UiTree {
    widgets: Vec<WidgetEntry>,
    area: Rect,
    gap: f64,
    direction: LayoutDirection,
    padding: EdgeInsets,
    align_items: CrossAlign,
    focus_index: Option<usize>,
}

impl UiTree {
    pub fn column(area: Rect, gap: f64) -> Self {
        Self {
            widgets: Vec::new(),
            area,
            gap,
            direction: LayoutDirection::Column,
            padding: EdgeInsets::all(0.0),
            align_items: CrossAlign::Stretch,
            focus_index: None,
        }
    }

    #[allow(dead_code)]
    pub fn row(area: Rect, gap: f64) -> Self {
        Self {
            widgets: Vec::new(),
            area,
            gap,
            direction: LayoutDirection::Row,
            padding: EdgeInsets::all(0.0),
            align_items: CrossAlign::Center,
            focus_index: None,
        }
    }

    #[allow(dead_code)]
    pub fn stack(area: Rect) -> Self {
        Self {
            widgets: Vec::new(),
            area,
            gap: 0.0,
            direction: LayoutDirection::Stack,
            padding: EdgeInsets::all(0.0),
            align_items: CrossAlign::Stretch,
            focus_index: None,
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(WidgetEntry {
            key: "",
            widget,
            layout: LayoutProps::auto(),
            focus_order: i32::MAX,
        });
    }

    #[allow(dead_code)]
    pub fn push_key(&mut self, key: &'static str, widget: Box<dyn Widget>) {
        self.widgets.push(WidgetEntry {
            key,
            widget,
            layout: LayoutProps::auto(),
            focus_order: i32::MAX,
        });
    }

    pub fn push_key_with(&mut self, key: &'static str, widget: Box<dyn Widget>, layout: LayoutProps) {
        self.widgets.push(WidgetEntry {
            key,
            widget,
            layout,
            focus_order: i32::MAX,
        });
    }

    pub fn push_key_with_order(
        &mut self,
        key: &'static str,
        widget: Box<dyn Widget>,
        layout: LayoutProps,
        focus_order: i32,
    ) {
        self.widgets.push(WidgetEntry {
            key,
            widget,
            layout,
            focus_order,
        });
    }

    pub fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    pub fn set_padding(&mut self, padding: EdgeInsets) {
        self.padding = padding;
    }

    pub fn set_align_items(&mut self, align: CrossAlign) {
        self.align_items = align;
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

        if pointer.focus_next && !self.focus_next_in_focused_child() {
            self.focus_next();
        } else if pointer.focus_prev && !self.focus_prev_in_focused_child() {
            self.focus_prev();
        }

        match self.direction {
            LayoutDirection::Column => self.draw_column(context, pointer),
            LayoutDirection::Row => self.draw_row(context, pointer),
            LayoutDirection::Stack => self.draw_stack(context, pointer),
        }
    }

    fn draw_column(
        &mut self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let inner = self.inner_area();

        let total_gap = self.gap * (self.widgets.len().saturating_sub(1) as f64);
        let mut fixed_height = 0.0;
        let mut total_flex = 0.0;
        for entry in &self.widgets {
            let (_, desired_h) = entry.widget.desired_size();
            match entry.layout.height {
                SizeSpec::Fixed(h) => fixed_height += h.max(0.0),
                SizeSpec::Auto => fixed_height += desired_h.max(0.0),
                SizeSpec::Flex(f) => total_flex += f.max(0.0),
            }
        }
        let remaining = (inner.height - fixed_height - total_gap).max(0.0);

        let mut y = inner.y;
        for (index, entry) in self.widgets.iter_mut().enumerate() {
            let (desired_w, desired_h) = entry.widget.desired_size();
            let height = match entry.layout.height {
                SizeSpec::Fixed(h) => h.max(0.0),
                SizeSpec::Auto => desired_h.max(0.0),
                SizeSpec::Flex(f) => {
                    if total_flex > 0.0 {
                        remaining * (f.max(0.0) / total_flex)
                    } else {
                        0.0
                    }
                }
            };

            let mut width = match entry.layout.width {
                SizeSpec::Fixed(w) => w.max(0.0).min(inner.width),
                SizeSpec::Auto => {
                    if desired_w > 0.0 {
                        desired_w.min(inner.width)
                    } else {
                        inner.width
                    }
                }
                SizeSpec::Flex(_) => inner.width,
            };
            let align = entry.layout.align_self.unwrap_or(self.align_items);
            if matches!(align, CrossAlign::Stretch) {
                width = inner.width;
            }
            let x = match align {
                CrossAlign::Start | CrossAlign::Stretch => inner.x,
                CrossAlign::Center => inner.x + (inner.width - width) * 0.5,
                CrossAlign::End => inner.x + inner.width - width,
            };

            let rect = Rect {
                x,
                y,
                width,
                height,
            };
            if (pointer.just_pressed || pointer.just_released)
                && rect.contains(pointer.x, pointer.y)
                && entry.widget.focusable()
            {
                self.focus_index = Some(index);
            }
            entry.widget.set_focused(self.focus_index == Some(index));
            entry.widget.set_rect(rect);
            events.extend(entry.widget.draw(context, pointer));
            if pointer.activate_primary && self.focus_index == Some(index) {
                if let Some(event) = entry.widget.activate() {
                    events.push(event);
                }
            }
            y += height + self.gap;
        }

        events
    }

    fn draw_row(
        &mut self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let inner = self.inner_area();
        let count = self.widgets.len();
        let total_gap = self.gap * (count.saturating_sub(1) as f64);

        let mut fixed_width = 0.0;
        let mut total_flex = 0.0;
        for entry in &self.widgets {
            let (desired_w, _) = entry.widget.desired_size();
            match entry.layout.width {
                SizeSpec::Fixed(w) => fixed_width += w.max(0.0),
                SizeSpec::Auto => fixed_width += desired_w.max(0.0),
                SizeSpec::Flex(f) => total_flex += f.max(0.0),
            }
        }

        let remaining = (inner.width - fixed_width - total_gap).max(0.0);

        let mut x = inner.x;
        for (index, entry) in self.widgets.iter_mut().enumerate() {
            let (desired_w, desired_h) = entry.widget.desired_size();
            let width = match entry.layout.width {
                SizeSpec::Fixed(w) => w.max(0.0),
                SizeSpec::Auto => desired_w.max(0.0),
                SizeSpec::Flex(f) => {
                    if total_flex > 0.0 {
                        remaining * (f.max(0.0) / total_flex)
                    } else {
                        0.0
                    }
                }
            };
            let mut height = match entry.layout.height {
                SizeSpec::Fixed(h) => h.max(0.0).min(inner.height),
                SizeSpec::Auto => {
                    if desired_h > 0.0 {
                        desired_h.min(inner.height)
                    } else {
                        inner.height
                    }
                }
                SizeSpec::Flex(_) => inner.height,
            };
            let align = entry.layout.align_self.unwrap_or(self.align_items);
            if matches!(align, CrossAlign::Stretch) {
                height = inner.height;
            }
            let y = match align {
                CrossAlign::Start | CrossAlign::Stretch => inner.y,
                CrossAlign::Center => inner.y + (inner.height - height) * 0.5,
                CrossAlign::End => inner.y + inner.height - height,
            };

            let rect = Rect {
                x,
                y,
                width,
                height,
            };
            if (pointer.just_pressed || pointer.just_released)
                && rect.contains(pointer.x, pointer.y)
                && entry.widget.focusable()
            {
                self.focus_index = Some(index);
            }
            entry.widget.set_focused(self.focus_index == Some(index));
            entry.widget.set_rect(rect);

            events.extend(entry.widget.draw(context, pointer));
            if pointer.activate_primary && self.focus_index == Some(index) {
                if let Some(event) = entry.widget.activate() {
                    events.push(event);
                }
            }

            x += width + self.gap;
        }

        events
    }

    fn draw_stack(
        &mut self,
        context: &CanvasRenderingContext2d,
        pointer: &PointerState,
    ) -> Vec<UiEvent> {
        let mut events = Vec::new();
        let inner = self.inner_area();

        for (index, entry) in self.widgets.iter_mut().enumerate() {
            let (desired_w, desired_h) = entry.widget.desired_size();
            let width = match entry.layout.width {
                SizeSpec::Fixed(w) => w.max(0.0).min(inner.width),
                SizeSpec::Auto => {
                    if desired_w > 0.0 {
                        desired_w.min(inner.width)
                    } else {
                        inner.width
                    }
                }
                SizeSpec::Flex(_) => inner.width,
            };
            let height = match entry.layout.height {
                SizeSpec::Fixed(h) => h.max(0.0).min(inner.height),
                SizeSpec::Auto => {
                    if desired_h > 0.0 {
                        desired_h.min(inner.height)
                    } else {
                        inner.height
                    }
                }
                SizeSpec::Flex(_) => inner.height,
            };
            let align = entry.layout.align_self.unwrap_or(self.align_items);
            let x = match align {
                CrossAlign::Start | CrossAlign::Stretch => inner.x,
                CrossAlign::Center => inner.x + (inner.width - width) * 0.5,
                CrossAlign::End => inner.x + inner.width - width,
            };
            let y = inner.y + (inner.height - height) * 0.5;

            let rect = Rect {
                x,
                y,
                width,
                height,
            };
            if (pointer.just_pressed || pointer.just_released)
                && rect.contains(pointer.x, pointer.y)
                && entry.widget.focusable()
            {
                self.focus_index = Some(index);
            }
            entry.widget.set_focused(self.focus_index == Some(index));
            entry.widget.set_rect(rect);
            events.extend(entry.widget.draw(context, pointer));
            if pointer.activate_primary && self.focus_index == Some(index) {
                if let Some(event) = entry.widget.activate() {
                    events.push(event);
                }
            }
        }

        events
    }

    fn inner_area(&self) -> Rect {
        let x = self.area.x + self.padding.left;
        let y = self.area.y + self.padding.top;
        let width = (self.area.width - self.padding.left - self.padding.right).max(0.0);
        let height = (self.area.height - self.padding.top - self.padding.bottom).max(0.0);
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn focus_next(&mut self) -> bool {
        if self.widgets.is_empty() {
            self.focus_index = None;
            return false;
        }

        let mut ordered = self
            .widgets
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.widget.focusable())
            .map(|(index, entry)| (entry.focus_order, index))
            .collect::<Vec<_>>();
        if ordered.is_empty() {
            self.focus_index = None;
            return false;
        }

        ordered.sort_by(|(a_order, a_index), (b_order, b_index)| {
            a_order.cmp(b_order).then(a_index.cmp(b_index))
        });

        let current = self.focus_index;
        let next_pos = ordered
            .iter()
            .position(|(_, index)| Some(*index) == current)
            .map(|pos| (pos + 1) % ordered.len())
            .unwrap_or(0);
        self.focus_index = Some(ordered[next_pos].1);
        true
    }

    pub fn focus_prev(&mut self) -> bool {
        if self.widgets.is_empty() {
            self.focus_index = None;
            return false;
        }

        let mut ordered = self
            .widgets
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.widget.focusable())
            .map(|(index, entry)| (entry.focus_order, index))
            .collect::<Vec<_>>();
        if ordered.is_empty() {
            self.focus_index = None;
            return false;
        }

        ordered.sort_by(|(a_order, a_index), (b_order, b_index)| {
            a_order.cmp(b_order).then(a_index.cmp(b_index))
        });

        let current = self.focus_index;
        let prev_pos = ordered
            .iter()
            .position(|(_, index)| Some(*index) == current)
            .map(|pos| if pos == 0 { ordered.len() - 1 } else { pos - 1 })
            .unwrap_or(ordered.len() - 1);
        self.focus_index = Some(ordered[prev_pos].1);
        true
    }

    pub fn focus_next_in_focused_child(&mut self) -> bool {
        match self.focus_index {
            Some(index) => self.widgets[index].widget.focus_next_in_children(),
            None => false,
        }
    }

    pub fn focus_prev_in_focused_child(&mut self) -> bool {
        match self.focus_index {
            Some(index) => self.widgets[index].widget.focus_prev_in_children(),
            None => false,
        }
    }
}
