use olive_rs::{save_to_png_file, save_to_ppm_file, Canvas, Pixel};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut pixels = [Pixel::new(0, 0, 0, 0); WIDTH * HEIGHT];
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut pixels);
    canvas.fill(Pixel::new(0xff, 0, 0, 0xff));
    save_to_ppm_file(&canvas, "output.ppm").unwrap();
    save_to_png_file(&canvas, "output.png").unwrap();
}
