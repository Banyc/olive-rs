use file_gen::{save_to_png_file, save_to_ppm_file};
use olive_rs::{Canvas, Pixel, StackPixels2D};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SIZE: usize = WIDTH * HEIGHT;

fn main() {
    let mut pixels = StackPixels2D::<SIZE>::new(WIDTH, HEIGHT, Pixel::new(0, 0, 0, 0));
    let mut canvas = Canvas::new_entire(&mut pixels);
    canvas.fill(Pixel::new(0xff, 0, 0, 0xff));
    save_to_ppm_file(&pixels, "output.ppm").unwrap();
    save_to_png_file(&pixels, "output.png").unwrap();
}
