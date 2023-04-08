use file_gen::{save_to_png_file, save_to_ppm_file};
use olive_rs::{Canvas, Pixel, Point};

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
    {
        let p1 = Point { x: 0, y: h };
        let p2 = Point { x: w, y: 0 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: w, y: h };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    // Horizontal and vertical lines
    {
        let p1 = Point { x: w / 2, y: h };
        let p2 = Point { x: w / 2, y: 0 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: 0, y: h / 2 };
        let p2 = Point { x: w, y: h / 2 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    // Diagonal lines of the half-size rectangles
    {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: w / 2, y: h };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: w / 2, y: h };
        let p2 = Point { x: w, y: 0 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: 0, y: h };
        let p2 = Point { x: w / 2, y: 0 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: w / 2, y: 0 };
        let p2 = Point { x: w, y: h };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: 0, y: h / 2 };
        let p2 = Point { x: w, y: 0 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: w, y: h / 2 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: 0, y: h / 2 };
        let p2 = Point { x: w, y: h };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = Point { x: 0, y: h };
        let p2 = Point { x: w, y: h / 2 };
        canvas.draw_line(p1, p2, FOREGROUND_COLOR);
    }
    save_to_ppm_file(&canvas, "lines.ppm").unwrap();
    save_to_png_file(&canvas, "lines.png").unwrap();
}
