use olive_rs::{save_to_ppm_file, Canvas, Pixel, Point};

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
            let u = x as f32 / COLS as f32;
            let v = y as f32 / ROWS as f32;
            let t = (u + v) / 2.;

            let color = Pixel::new(0x80, 0x80, 0x80, 0xff);
            let w = WIDTH / COLS;
            let h = HEIGHT / ROWS;
            let r = (w).min(h);

            let r = lerp((r / 8) as f32, (r / 2) as f32, t);
            let c = Point {
                x: (x * w + w / 2) as isize,
                y: (y * h + h / 2) as isize,
            };
            canvas.fill_circle(c, r as isize, color);
        }
    }

    save_to_ppm_file(&canvas, "circle.ppm").unwrap();
}

/// Linear interpolation between two values.
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    assert!(t >= 0.);
    assert!(t <= 1.);
    a + (b - a) * t
}
