#[cfg(test)]
mod tests {
    use std::{io::Read, path::Path};

    use olive_rs::{save_to_ppm_stream, Canvas, Pixel};

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
        canvas.fill_rect(w / 2 - w / 8, h / 2 - h / 8, w / 4, h / 4, RED_COLOR);
        canvas.fill_rect(w - 1, h - 1, -w / 2, -h / 2, GREEN_COLOR);
        canvas.fill_rect(-w / 4, -h / 4, w / 2, h / 2, BLUE_COLOR);
        assert_eq_canvas_with_file("tests/assets/fill_rect.ppm", &canvas);
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
        canvas.fill_circle(0, 0, w / 2, RED_COLOR);
        canvas.fill_circle(w / 2, h / 2, w / 4, BLUE_COLOR);
        canvas.fill_circle(w * 3 / 4, h * 3 / 4, -w / 4, GREEN_COLOR);
        assert_eq_canvas_with_file("tests/assets/fill_circle.ppm", &canvas);
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
        canvas.draw_line(0, 0, w, h, RED_COLOR);
        canvas.draw_line(w, 0, 0, h, BLUE_COLOR);
        canvas.draw_line(w / 2, 0, w / 2, h, GREEN_COLOR);
        assert_eq_canvas_with_file("tests/assets/draw_line.ppm", &canvas);
    }
}
