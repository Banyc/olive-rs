use olive_rs::{save_to_ppm_file, Canvas, Pixel};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);
const FOREGROUND_COLOR: Pixel = Pixel::new(0xff, 0xff, 0xff, 0xff);

fn main() {
    let mut pixels = [Pixel::new(0, 0, 0, 0); WIDTH * HEIGHT];
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut pixels);
    canvas.fill(BACKGROUND_COLOR);
    // Diagonal lines of the full-size rectangle
    canvas.draw_line(0, HEIGHT, WIDTH, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, 0, WIDTH, HEIGHT, FOREGROUND_COLOR);
    // Horizontal and vertical lines
    canvas.draw_line(WIDTH / 2, HEIGHT, WIDTH / 2, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, HEIGHT / 2, WIDTH, HEIGHT / 2, FOREGROUND_COLOR);
    // Diagonal lines of the half-size rectangles
    canvas.draw_line(0, 0, WIDTH / 2, HEIGHT, FOREGROUND_COLOR);
    canvas.draw_line(WIDTH / 2, HEIGHT, WIDTH, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, HEIGHT, WIDTH / 2, 0, FOREGROUND_COLOR);
    canvas.draw_line(WIDTH / 2, 0, WIDTH, HEIGHT, FOREGROUND_COLOR);
    canvas.draw_line(0, HEIGHT / 2, WIDTH, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, 0, WIDTH, HEIGHT / 2, FOREGROUND_COLOR);
    canvas.draw_line(0, HEIGHT / 2, WIDTH, HEIGHT, FOREGROUND_COLOR);
    canvas.draw_line(0, HEIGHT, WIDTH, HEIGHT / 2, FOREGROUND_COLOR);
    save_to_ppm_file(&canvas, "lines.ppm").unwrap();
}
