use file_gen::{save_to_png_file, save_to_ppm_file};
use olive_rs::{Canvas, Pixel, PixelPoint, StackPixels2D};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SIZE: usize = WIDTH * HEIGHT;

const COLS: usize = 8;
const ROWS: usize = 6;

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);

fn main() {
    let mut pixels = StackPixels2D::<SIZE>::new(WIDTH, HEIGHT, Pixel::new(0, 0, 0, 0));
    let mut canvas = Canvas::new(&mut pixels);
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
            let p = PixelPoint {
                x: (x * rw) as isize,
                y: (y * rh) as isize,
            };
            canvas.fill_pixel_rect(p, rw as isize, rh as isize, color);
        }
    }

    save_to_ppm_file(&pixels, "checker.ppm").unwrap();
    save_to_png_file(&pixels, "checker.png").unwrap();
}
