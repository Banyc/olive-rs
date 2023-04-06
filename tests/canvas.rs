#[cfg(test)]
mod tests {
    use std::{io::Read, path::Path};

    use olive_rs::{save_to_ppm_stream, Canvas, Pixel, Point};

    const BACKGROUND_COLOR: Pixel = Pixel::new(0x20, 0x20, 0x20, 0xff);
    const RED_COLOR: Pixel = Pixel::new(0xff, 0, 0, 0xff);
    const GREEN_COLOR: Pixel = Pixel::new(0, 0xff, 0, 0xff);
    const BLUE_COLOR: Pixel = Pixel::new(0, 0, 0xff, 0xff);

    fn assert_eq_bytes_with_file<P>(expected: P, actual: &[u8])
    where
        P: AsRef<Path>,
    {
        let file_path = expected.as_ref();
        let mut file = std::fs::File::open(file_path).unwrap();
        let mut expected = Vec::new();
        file.read_to_end(&mut expected).unwrap();
        assert_eq!(actual, expected);
    }

    fn assert_eq_canvas_with_file<P>(expected: P, actual: &Canvas)
    where
        P: AsRef<Path>,
    {
        let mut bytes = Vec::new();
        save_to_ppm_stream(actual, &mut bytes).unwrap();
        assert_eq_bytes_with_file(expected, &bytes);
    }

    #[test]
    fn fill_rect() {
        let w = 128;
        let h = 128;
        let mut pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let mut canvas = Canvas::new(w, h, &mut pixels);
        canvas.fill(BACKGROUND_COLOR);
        let w = w as isize;
        let h = h as isize;
        {
            let p = Point {
                x: w / 2 - w / 8,
                y: h / 2 - h / 8,
            };
            canvas.fill_rect(p, w / 4, h / 4, RED_COLOR);
        }
        {
            let p = Point { x: w - 1, y: h - 1 };
            canvas.fill_rect(p, -w / 2, -h / 2, GREEN_COLOR);
        }
        {
            let p = Point {
                x: -w / 4,
                y: -h / 4,
            };
            canvas.fill_rect(p, w / 2, h / 2, BLUE_COLOR);
        }
        assert_eq_canvas_with_file("tests/assets/fill_rect.ppm", &canvas);
    }

    #[test]
    fn zero_size_rect() {
        let w = 1;
        let h = 1;
        let mut pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let mut canvas = Canvas::new(w, h, &mut pixels);
        canvas.fill(BACKGROUND_COLOR);
        {
            let p = Point { x: 0, y: 0 };
            canvas.fill_rect(p, 0, 0, RED_COLOR);
        }
        assert_eq!(canvas.pixels(), [BACKGROUND_COLOR]);
    }

    #[test]
    fn fill_circle() {
        let w = 128;
        let h = 128;
        let mut pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let mut canvas = Canvas::new(w, h, &mut pixels);
        canvas.fill(BACKGROUND_COLOR);
        let w = w as isize;
        let h = h as isize;
        {
            let c = Point { x: 0, y: 0 };
            canvas.fill_circle(c, w / 2, RED_COLOR);
        }
        {
            let c = Point { x: w / 2, y: h / 2 };
            canvas.fill_circle(c, w / 4, BLUE_COLOR);
        }
        {
            let c = Point {
                x: w * 3 / 4,
                y: h * 3 / 4,
            };
            canvas.fill_circle(c, -w / 4, GREEN_COLOR);
        }
        assert_eq_canvas_with_file("tests/assets/fill_circle.ppm", &canvas);
    }

    #[test]
    fn zero_radius_circle() {
        let w = 1;
        let h = 1;
        let mut pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let mut canvas = Canvas::new(w, h, &mut pixels);
        canvas.fill(BACKGROUND_COLOR);
        {
            let c = Point { x: 0, y: 0 };
            canvas.fill_circle(c, 0, RED_COLOR);
        }
        assert_eq!(canvas.pixels(), [BACKGROUND_COLOR]);
    }

    #[test]
    fn draw_line() {
        let w = 128;
        let h = 128;
        let mut pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let mut canvas = Canvas::new(w, h, &mut pixels);
        canvas.fill(BACKGROUND_COLOR);
        let w = w as isize;
        let h = h as isize;
        {
            let p1 = Point { x: 0, y: 0 };
            let p2 = Point { x: w, y: h };
            canvas.draw_line(p1, p2, RED_COLOR);
        }
        {
            let p1 = Point { x: w, y: 0 };
            let p2 = Point { x: 0, y: h };
            canvas.draw_line(p1, p2, BLUE_COLOR);
        }
        {
            let p1 = Point { x: w / 2, y: 0 };
            let p2 = Point { x: w / 2, y: h };
            canvas.draw_line(p1, p2, GREEN_COLOR);
        }
        assert_eq_canvas_with_file("tests/assets/draw_line.ppm", &canvas);
    }

    #[test]
    fn fill_triangle() {
        let w = 128;
        let h = 128;
        let mut pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let mut canvas = Canvas::new(w, h, &mut pixels);
        canvas.fill(BACKGROUND_COLOR);
        let w = w as isize;
        let h = h as isize;
        {
            let v1 = Point { x: w / 2, y: h / 8 };
            let v2 = Point { x: w / 8, y: h / 2 };
            let v3 = Point {
                x: w * 7 / 8,
                y: h * 7 / 8,
            };
            canvas.fill_triangle(v1, v2, v3, RED_COLOR);
        }
        {
            let v1 = Point {
                x: w / 2,
                y: h * 2 / 8,
            };
            let v2 = Point {
                x: w * 2 / 8,
                y: h / 2,
            };
            let v3 = Point {
                x: w * 6 / 8,
                y: h / 2,
            };
            canvas.fill_triangle(v1, v2, v3, GREEN_COLOR);
        }
        {
            let v1 = Point { x: w / 8, y: h / 8 };
            let v2 = Point {
                x: w / 8,
                y: h * 3 / 8,
            };
            let v3 = Point {
                x: w * 3 / 8,
                y: h * 3 / 8,
            };
            canvas.fill_triangle(v1, v2, v3, BLUE_COLOR);
        }
        assert_eq_canvas_with_file("tests/assets/fill_triangle.ppm", &canvas);
    }

    #[test]
    fn alpha_blending() {
        let w = 128;
        let h = 128;
        let mut pixels = vec![Pixel::new(0, 0, 0, 0); w * h];
        let mut canvas = Canvas::new(w, h, &mut pixels);
        canvas.fill(BACKGROUND_COLOR);
        let w = w as isize;
        let h = h as isize;
        {
            let p = Point { x: 0, y: 0 };
            canvas.fill_rect(p, w * 3 / 4, h * 3 / 4, RED_COLOR);
        }
        {
            let p = Point { x: w - 1, y: h - 1 };
            canvas.fill_rect(p, -w * 3 / 4, -h * 3 / 4, Pixel::new(0, 0xaa, 0, 0x55));
        }
        {
            let c = Point { x: w / 2, y: h / 2 };
            canvas.fill_circle(c, w / 4, Pixel::new(0, 0, 0xaa, 0xbb));
        }
        {
            let v1 = Point { x: 0, y: h - 1 };
            let v2 = Point { x: w - 1, y: h - 1 };
            let v3 = Point { x: w / 2, y: 0 };
            canvas.fill_triangle(v1, v2, v3, Pixel::new(0xaa, 0xaa, 0, 0xbb));
        }
        assert_eq_canvas_with_file("tests/assets/alpha_blending.ppm", &canvas);
    }
}
