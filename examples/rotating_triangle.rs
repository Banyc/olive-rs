use std::f64::consts::PI;

use olive_rs::{Canvas, Pixel, Point, Render};
use wasm::start_render;

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);
const RED_COLOR: Pixel = Pixel::new(0xff, 0, 0, 0xff);
const CIRCLE_COLOR: Pixel = Pixel::new(0, 0, 0xaa, 0x99);

fn main() {
    let w = 960;
    let h = 720;

    let animation = Animation::new(w, h);
    start_render(w as u32, h as u32, animation);
}

pub struct Animation {
    pixels: Vec<Pixel>,
    w: usize,
    h: usize,
    rotating_triangle: RotatingTriangle,
    bouncing_circle: BouncingCircle,
}

impl Animation {
    pub fn new(w: usize, h: usize) -> Self {
        let pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let w_i = w as isize;
        let h_i = h as isize;
        let rotating_triangle = RotatingTriangle::new(
            Point {
                x: w_i / 2,
                y: h_i / 8,
            },
            Point {
                x: w_i / 8,
                y: h_i / 2,
            },
            Point {
                x: w_i * 7 / 8,
                y: h_i * 7 / 8,
            },
            RED_COLOR,
            0.0,
        );
        let bouncing_circle = BouncingCircle::new(
            Point {
                x: w_i / 2,
                y: h_i / 2,
            },
            100,
            100,
            100,
            CIRCLE_COLOR,
        );
        Self {
            pixels,
            w,
            h,
            rotating_triangle,
            bouncing_circle,
        }
    }
}

impl Render for Animation {
    fn render(&mut self, dt_ms: f64) {
        let dt_s = dt_ms * 0.001;
        let mut canvas = Canvas::new(self.w, self.h, &mut self.pixels);
        canvas.fill(BACKGROUND_COLOR);
        self.rotating_triangle.render(&mut canvas, dt_s);
        self.bouncing_circle.render(&mut canvas, dt_s);
    }

    fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }
}

pub struct RotatingTriangle {
    v1: Point,
    v2: Point,
    v3: Point,
    color: Pixel,
    angle: f64,
}

impl RotatingTriangle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(v1: Point, v2: Point, v3: Point, color: Pixel, angle: f64) -> Self {
        Self {
            v1,
            v2,
            v3,
            color,
            angle,
        }
    }
}

impl RotatingTriangle {
    fn render(&mut self, canvas: &mut Canvas<'_>, dt_s: f64) {
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

        self.angle += 0.5 * PI * dt_s;

        let c = Point {
            x: (canvas.width() / 2) as isize,
            y: (canvas.height() / 2) as isize,
        };
        let v1 = rotate_point(c, self.v1, self.angle);
        let v2 = rotate_point(c, self.v2, self.angle);
        let v3 = rotate_point(c, self.v3, self.angle);

        canvas.fill_triangle(v1, v2, v3, self.color);
    }
}

pub struct BouncingCircle {
    c: Point,
    r: isize,
    dx_coe: isize,
    dy_coe: isize,
    color: Pixel,
}

impl BouncingCircle {
    pub fn new(c: Point, r: isize, dx_coe: isize, dy_coe: isize, color: Pixel) -> Self {
        Self {
            c,
            r,
            dx_coe,
            dy_coe,
            color,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<'_>, dt_s: f64) {
        let w = canvas.width() as isize;
        let h = canvas.height() as isize;

        let dx = (dt_s * self.dx_coe as f64) as isize;
        let dy = (dt_s * self.dy_coe as f64) as isize;
        let x = self.c.x + dx;
        let y = self.c.y + dy;

        if x - self.r < 0 || x + self.r > w {
            self.dx_coe = -self.dx_coe;
        } else {
            self.c.x = x;
        }
        if y - self.r < 0 || y + self.r > h {
            self.dy_coe = -self.dy_coe;
        } else {
            self.c.y = y;
        }

        canvas.fill_circle(self.c, self.r, self.color);
    }
}
