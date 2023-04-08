use file_gen::{save_to_png_file, save_to_ppm_file};
use olive_rs::{Canvas, Pixel, PointF};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const COLS: usize = 8;
const ROWS: usize = 6;

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);

fn main() {
    let mut pixels = [Pixel::new(0, 0, 0, 0); WIDTH * HEIGHT];
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut pixels);
    canvas.fill(BACKGROUND_COLOR);

    for y in 0..ROWS {
        for x in 0..COLS {
            let u = x as f64 / COLS as f64;
            let v = y as f64 / ROWS as f64;
            let t = (u + v) / 2.;

            let color = Pixel::new(0x80, 0x80, 0x80, 0xff);
            let w = WIDTH / COLS;
            let h = HEIGHT / ROWS;
            let r = (w).min(h) as f64;

            let r = lerp(r / 8., r / 2., t);
            let c = PointF::from_int((x * w + w / 2) as isize, (y * h + h / 2) as isize);
            canvas.fill_circle(c, r, color);
        }
    }

    save_to_ppm_file(&canvas, "circle.ppm").unwrap();
    save_to_png_file(&canvas, "circle.png").unwrap();
}

/// Linear interpolation between two values.
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    assert!(t >= 0.);
    assert!(t <= 1.);
    a + (b - a) * t
}
