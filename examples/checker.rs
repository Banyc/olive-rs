use olive_rs::{save_to_png_file, save_to_ppm_file, Canvas, Pixel, Point};

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
            let color = if (x + y) % 2 == 0 {
                Pixel::new(0xff, 0xff, 0xff, 0xff)
            } else {
                Pixel::new(0x80, 0x80, 0x80, 0xff)
            };
            let rw = WIDTH / COLS;
            let rh = HEIGHT / ROWS;
            let p = Point {
                x: (x * rw) as isize,
                y: (y * rh) as isize,
            };
            canvas.fill_rect(p, rw as isize, rh as isize, color);
        }
    }

    save_to_ppm_file(&canvas, "checker.ppm").unwrap();
    save_to_png_file(&canvas, "checker.png").unwrap();
}
