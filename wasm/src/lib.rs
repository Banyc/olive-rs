use std::{cell::RefCell, rc::Rc};

use olive_rs::{Pixel, Render};
use wasm_bindgen::prelude::*;

const CANVAS_ID: &str = "app";

pub fn start_render<R>(w: u32, h: u32, mut render: R)
where
    R: Render + 'static,
{
    setup(w, h);

    let mut render_time_ms: Option<f64> = None;

    start_loop(move |timestamp_ms| {
        let dt_ms = match render_time_ms {
            Some(t) => timestamp_ms - t,
            None => 0.,
        };
        render_time_ms = Some(timestamp_ms);
        render.render(dt_ms);
        let pixels = render.pixels();

        draw(pixels, w, h);
    });
}

/// Ref: <https://github.com/takahirox/ecs-rust/blob/f62c0a57409c494c4b85e5a320ca5bda74e78c8e/web/examples/canvas_breakout/src/lib.rs#L474-L482>
fn start_loop<F>(mut f: F)
where
    F: FnMut(f64) + 'static,
{
    let a = Rc::new(RefCell::new(None));
    let b = a.clone();

    fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .unwrap();
    }

    *b.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        f(timestamp);
        request_animation_frame(a.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(b.borrow().as_ref().unwrap());
}

fn setup(w: u32, h: u32) {
    // Set title
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_title("Olive-rs");

    // Create canvas
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap();
    canvas.set_id(CANVAS_ID);
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(w);
    canvas.set_height(h);

    // Append the canvas to the DOM
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&canvas)
        .unwrap();
}

fn draw(pixels: &[Pixel], w: u32, h: u32) {
    let app = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(CANVAS_ID)
        .unwrap();
    let app = app.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let ctx = app.get_context("2d").unwrap().unwrap();
    let ctx = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    let pixels_u8 =
        unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const u8, pixels.len() * 4) };
    let img_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(pixels_u8),
        w,
        h,
    )
    .unwrap();
    ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
}
