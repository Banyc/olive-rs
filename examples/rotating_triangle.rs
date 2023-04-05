use std::f32::consts::PI;

use olive_rs::{Canvas, Pixel, Point};
use wasm::{draw, setup, start_loop};

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);
const RED_COLOR: Pixel = Pixel::new(0xff, 0, 0, 0xff);

fn main() {
    let w = 800;
    let h = 600;

    setup(w as u32, h as u32);

    let w = w as isize;
    let h = h as isize;
    let mut rotating_triangle = RotatingTriangle::new(
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

    let mut render_time: Option<f64> = None;

    start_loop(move |timestamp| {
        let dt = match render_time {
            Some(t) => timestamp - t,
            None => 0.0,
        };
        render_time = Some(timestamp);
        rotating_triangle.render(0.001 * dt as f32);
        let pixels = rotating_triangle.pixels();

        draw(pixels, w as u32, h as u32);
    });
}

pub struct RotatingTriangle {
    pixels: Vec<Pixel>,
    w: usize,
    h: usize,
    v1: Point,
    v2: Point,
    v3: Point,
    color: Pixel,
    angle: f32,
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
        angle: f32,
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

    pub fn render(&mut self, delta: f32) {
        self.angle += 2. * PI * delta;

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

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }
}

fn rotate_point(c: Point, p: Point, angle: f32) -> Point {
    // Vector from center to point
    let vx = p.x - c.x;
    let vy = p.y - c.y;
    let vx = vx as f32;
    let vy = vy as f32;

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
