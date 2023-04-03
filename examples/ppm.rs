use olive_rs::{save_to_ppm_file, Pixel, Pixels};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut pixels = [Pixel::new(0, 0, 0, 0); WIDTH * HEIGHT];
    let mut pixels = Pixels::new(WIDTH, HEIGHT, &mut pixels);
    pixels.fill(Pixel::new(0xff, 0, 0, 0xff));
    save_to_ppm_file(&pixels, "output.ppm").unwrap();
}
