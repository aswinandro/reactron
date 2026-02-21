use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, PointerEvent};

pub fn sync_canvas_resolution(canvas: &HtmlCanvasElement, dpr: f64) -> (f64, f64) {
    let css_width = canvas.client_width().max(1) as f64;
    let css_height = canvas.client_height().max(1) as f64;

    let next_width = (css_width * dpr).round().max(1.0) as u32;
    let next_height = (css_height * dpr).round().max(1.0) as u32;

    if canvas.width() != next_width {
        canvas.set_width(next_width);
    }
    if canvas.height() != next_height {
        canvas.set_height(next_height);
    }

    (f64::from(canvas.width()), f64::from(canvas.height()))
}

pub fn pointer_position_in_canvas(event: &PointerEvent, canvas: &HtmlCanvasElement) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let scale_x = if rect.width() > 0.0 {
        f64::from(canvas.width()) / rect.width()
    } else {
        1.0
    };
    let scale_y = if rect.height() > 0.0 {
        f64::from(canvas.height()) / rect.height()
    } else {
        1.0
    };

    let x = (f64::from(event.client_x()) - rect.left()) * scale_x;
    let y = (f64::from(event.client_y()) - rect.top()) * scale_y;
    (x, y)
}

pub fn clear(context: &CanvasRenderingContext2d, width: f64, height: f64, color: &str) {
    context.set_fill_style_str(color);
    context.fill_rect(0.0, 0.0, width, height);
}
