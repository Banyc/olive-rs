use std::f64::consts::PI;

use olive_rs::{Canvas, EvenF, HeapPixels2D, Pixel, PixelPointF, Pixels2D, Render};
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
    pixels: HeapPixels2D,
    rotating_triangle: RotatingTriangle,
    bouncing_circle: BouncingCircle,
}

impl Animation {
    pub fn new(w: usize, h: usize) -> Self {
        let pixels = HeapPixels2D::new(w, h, Pixel::new(0, 0, 0, 0));
        let w_i = w as isize;
        let h_i = h as isize;
        let rotating_triangle = RotatingTriangle::new(
            PixelPointF::from_int(w_i / 2, h_i / 8),
            PixelPointF::from_int(w_i / 8, h_i / 2),
            PixelPointF::from_int(w_i * 7 / 8, h_i * 7 / 8),
            RED_COLOR,
            0.0,
        );
        let bouncing_circle = BouncingCircle::new(
            PixelPointF::from_int(w_i / 2, h_i / 2),
            100.,
            100.,
            100.,
            CIRCLE_COLOR,
        );
        Self {
            pixels,
            rotating_triangle,
            bouncing_circle,
        }
    }
}

impl Render for Animation {
    fn render(&mut self, dt_ms: f64) {
        let dt_s = dt_ms * 0.001;
        let mut canvas = Canvas::new_entire(&mut self.pixels);
        canvas.fill(BACKGROUND_COLOR);
        self.rotating_triangle.render(&mut canvas, dt_s);
        self.bouncing_circle.render(&mut canvas, dt_s);
    }

    fn pixels(&self) -> &[Pixel] {
        self.pixels.pixels()
    }
}

pub struct RotatingTriangle {
    v1: PixelPointF,
    v2: PixelPointF,
    v3: PixelPointF,
    color: Pixel,
    angle: f64,
}

impl RotatingTriangle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        v1: PixelPointF,
        v2: PixelPointF,
        v3: PixelPointF,
        color: Pixel,
        angle: f64,
    ) -> Self {
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
    fn render<CP>(&mut self, canvas: &mut Canvas<'_, CP>, dt_s: f64)
    where
        CP: Pixels2D,
    {
        fn rotate_point(c: PixelPointF, p: PixelPointF, angle: f64) -> PixelPointF {
            // Vector from center to point
            let vx = p.x() - c.x();
            let vy = p.y() - c.y();
            let vx = vx.to_f();
            let vy = vy.to_f();

            // Rotate vector
            let mag = (vx * vx + vy * vy).sqrt();
            let dir = vy.atan2(vx) + angle;
            let vx = dir.cos() * mag;
            let vy = dir.sin() * mag;

            // Rotated point
            PixelPointF::new(c.x().add_f(vx), c.y().add_f(vy))
        }

        self.angle += 0.5 * PI * dt_s;

        let c = PixelPointF::from_int(
            (canvas.inner().width() / 2) as isize,
            (canvas.inner().height() / 2) as isize,
        );
        let v1 = rotate_point(c, self.v1, self.angle);
        let v2 = rotate_point(c, self.v2, self.angle);
        let v3 = rotate_point(c, self.v3, self.angle);

        canvas.fill_pixel_triangle(v1, v2, v3, self.color);
    }
}

pub struct BouncingCircle {
    c: PixelPointF,
    r: f64,
    vx: f64,
    vy: f64,
    color: Pixel,
}

impl BouncingCircle {
    pub fn new(c: PixelPointF, r: f64, vx: f64, vy: f64, color: Pixel) -> Self {
        Self {
            c,
            r,
            vx,
            vy,
            color,
        }
    }

    pub fn render<CP>(&mut self, canvas: &mut Canvas<'_, CP>, dt_s: f64)
    where
        CP: Pixels2D,
    {
        let w = canvas.inner().width() as isize;
        let h = canvas.inner().height() as isize;
        let w = EvenF::new(w, 0.);
        let h = EvenF::new(h, 0.);

        let dx = dt_s * self.vx;
        let dy = dt_s * self.vy;
        let x = self.c.x().add_f(dx);
        let y = self.c.y().add_f(dy);

        if x.add_f(-self.r) < EvenF::zero() || x.add_f(self.r) > w {
            self.vx = -self.vx;
        } else {
            self.c = PixelPointF::new(x, self.c.y());
        }
        if y.add_f(-self.r) < EvenF::zero() || y.add_f(self.r) > h {
            self.vy = -self.vy;
        } else {
            self.c = PixelPointF::new(self.c.x(), y);
        }

        canvas.fill_pixel_circle(self.c, self.r, self.color);
    }
}
