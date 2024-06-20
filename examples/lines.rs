use file_gen::{save_to_png_file, save_to_ppm_file};
use olive_rs::{Canvas, Pixel, PixelPoint, StackPixels2D};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SIZE: usize = WIDTH * HEIGHT;

const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);
const FOREGROUND_COLOR: Pixel = Pixel::new(0xff, 0xff, 0xff, 0xff);

fn main() {
    let mut pixels = StackPixels2D::<SIZE>::new(WIDTH, HEIGHT, Pixel::new(0, 0, 0, 0));
    let mut canvas = Canvas::new_entire(&mut pixels);
    canvas.fill(BACKGROUND_COLOR);
    let w = WIDTH as isize;
    let h = HEIGHT as isize;
    // Diagonal lines of the full-size rectangle
    {
        let p1 = PixelPoint { x: 0, y: h };
        let p2 = PixelPoint { x: w, y: 0 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: 0, y: 0 };
        let p2 = PixelPoint { x: w, y: h };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    // Horizontal and vertical lines
    {
        let p1 = PixelPoint { x: w / 2, y: h };
        let p2 = PixelPoint { x: w / 2, y: 0 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: 0, y: h / 2 };
        let p2 = PixelPoint { x: w, y: h / 2 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    // Diagonal lines of the half-size rectangles
    {
        let p1 = PixelPoint { x: 0, y: 0 };
        let p2 = PixelPoint { x: w / 2, y: h };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: w / 2, y: h };
        let p2 = PixelPoint { x: w, y: 0 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: 0, y: h };
        let p2 = PixelPoint { x: w / 2, y: 0 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: w / 2, y: 0 };
        let p2 = PixelPoint { x: w, y: h };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: 0, y: h / 2 };
        let p2 = PixelPoint { x: w, y: 0 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: 0, y: 0 };
        let p2 = PixelPoint { x: w, y: h / 2 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: 0, y: h / 2 };
        let p2 = PixelPoint { x: w, y: h };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    {
        let p1 = PixelPoint { x: 0, y: h };
        let p2 = PixelPoint { x: w, y: h / 2 };
        canvas.draw_pixel_line(p1, p2, FOREGROUND_COLOR);
    }
    save_to_ppm_file(&pixels, "lines.ppm").unwrap();
    save_to_png_file(&pixels, "lines.png").unwrap();
}
