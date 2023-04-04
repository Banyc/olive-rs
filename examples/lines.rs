use olive_rs::{save_to_ppm_file, Canvas, Pixel};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);
const FOREGROUND_COLOR: Pixel = Pixel::new(0xff, 0xff, 0xff, 0xff);

fn main() {
    let mut pixels = [Pixel::new(0, 0, 0, 0); WIDTH * HEIGHT];
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut pixels);
    canvas.fill(BACKGROUND_COLOR);
    let w = WIDTH as isize;
    let h = HEIGHT as isize;
    // Diagonal lines of the full-size rectangle
    canvas.draw_line(0, h, w, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, 0, w, h, FOREGROUND_COLOR);
    // Horizontal and vertical lines
    canvas.draw_line(w / 2, h, w / 2, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, h / 2, w, h / 2, FOREGROUND_COLOR);
    // Diagonal lines of the half-size rectangles
    canvas.draw_line(0, 0, w / 2, h, FOREGROUND_COLOR);
    canvas.draw_line(w / 2, h, w, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, h, w / 2, 0, FOREGROUND_COLOR);
    canvas.draw_line(w / 2, 0, w, h, FOREGROUND_COLOR);
    canvas.draw_line(0, h / 2, w, 0, FOREGROUND_COLOR);
    canvas.draw_line(0, 0, w, h / 2, FOREGROUND_COLOR);
    canvas.draw_line(0, h / 2, w, h, FOREGROUND_COLOR);
    canvas.draw_line(0, h, w, h / 2, FOREGROUND_COLOR);
    save_to_ppm_file(&canvas, "lines.ppm").unwrap();
}
