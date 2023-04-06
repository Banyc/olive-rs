use std::f64::consts::PI;

use olive_rs::{Canvas, Pixel, Point, Render};
use wasm::start_render;

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);
const RED_COLOR: Pixel = Pixel::new(0xff, 0, 0, 0xff);

fn main() {
    let w = 800;
    let h = 600;

    let w = w as isize;
    let h = h as isize;
    let rotating_triangle = RotatingTriangle::new(
        w as usize,
        h as usize,
        Point { x: w / 2, y: h / 8 },
        Point { x: w / 8, y: h / 2 },
        Point {
            x: w * 7 / 8,
            y: h * 7 / 8,
        },
        RED_COLOR,
        0.0,
    );

    start_render(w as u32, h as u32, rotating_triangle);
}

pub struct RotatingTriangle {
    pixels: Vec<Pixel>,
    w: usize,
    h: usize,
    v1: Point,
    v2: Point,
    v3: Point,
    color: Pixel,
    angle: f64,
}

impl RotatingTriangle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        w: usize,
        h: usize,
        v1: Point,
        v2: Point,
        v3: Point,
        color: Pixel,
        angle: f64,
    ) -> Self {
        let pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        Self {
            pixels,
            w,
            h,
            v1,
            v2,
            v3,
            color,
            angle,
        }
    }
}

impl Render for RotatingTriangle {
    fn render(&mut self, dt_ms: f64) {
        self.angle += 2. * PI * dt_ms * 0.001;

        let mut canvas = Canvas::new(self.w, self.h, &mut self.pixels);

        let c = Point {
            x: (canvas.width() / 2) as isize,
            y: (canvas.height() / 2) as isize,
        };
        let v1 = rotate_point(c, self.v1, self.angle);
        let v2 = rotate_point(c, self.v2, self.angle);
        let v3 = rotate_point(c, self.v3, self.angle);

        canvas.fill(BACKGROUND_COLOR);
        canvas.fill_triangle(v1, v2, v3, self.color);
    }

    fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }
}

fn rotate_point(c: Point, p: Point, angle: f64) -> Point {
    // Vector from center to point
    let vx = p.x - c.x;
    let vy = p.y - c.y;
    let vx = vx as f64;
    let vy = vy as f64;

    // Rotate vector
    let mag = (vx * vx + vy * vy).sqrt();
    let dir = vy.atan2(vx) + angle;
    let vx = dir.cos() * mag;
    let vy = dir.sin() * mag;

    // Rotated point
    Point {
        x: c.x + vx as isize,
        y: c.y + vy as isize,
    }
}
