#[derive(Debug, PartialEq, Eq)]
pub struct Canvas<'vec> {
    width: usize,
    height: usize,
    pixels: &'vec mut [Pixel],
}

impl Canvas<'_> {
    pub fn new(width: usize, height: usize, pixels: &mut [Pixel]) -> Canvas {
        assert_eq!(width * height, pixels.len());
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &[Pixel] {
        self.pixels
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.pixels[y * self.width + x]
    }

    pub fn fill(&mut self, pixel: Pixel) {
        for p in self.pixels.iter_mut() {
            *p = pixel;
        }
    }

    pub fn fill_rect(&mut self, p: Point, w: isize, h: isize, color: Pixel) {
        let y_min = p.y.min(p.y + h).max(0) as usize;
        let y_max = p.y.max(p.y + h).max(0) as usize;
        let x_min = p.x.min(p.x + w).max(0) as usize;
        let x_max = p.x.max(p.x + w).max(0) as usize;
        for y in y_min..y_max {
            if y >= self.height {
                break;
            }
            for x in x_min..x_max {
                if x >= self.width {
                    break;
                }
                *self.pixel_mut(x, y) = color;
            }
        }
    }

    pub fn fill_circle(&mut self, c: Point, r: isize, color: Pixel) {
        let x1 = c.x - r;
        let x2 = c.x + r;
        let y1 = c.y - r;
        let y2 = c.y + r;
        let x_min = x1.min(x2).max(0);
        let x_max = x1.max(x2).max(0);
        let y_min = y1.min(y2).max(0);
        let y_max = y1.max(y2).max(0);
        for y in y_min..y_max {
            if y as usize >= self.height {
                break;
            }
            for x in x_min..x_max {
                if x as usize >= self.width {
                    break;
                }
                let dx = isize::abs_diff(x, c.x);
                let dy = isize::abs_diff(y, c.y);
                if dx * dx + dy * dy <= (r * r) as usize {
                    *self.pixel_mut(x as usize, y as usize) = color;
                }
            }
        }
    }

    pub fn draw_line(&mut self, p1: Point, p2: Point, color: Pixel) {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        let y_min = p1.y.min(p2.y).max(0);
        let y_max = p1.y.max(p2.y).max(0);
        let x_min = p1.x.min(p2.x).max(0);
        let x_max = p1.x.max(p2.x).max(0);

        if dx != 0 {
            let k = dy as f32 / dx as f32;
            let b = p1.y as f32 - k * p1.x as f32;
            for x in x_min..x_max {
                if x as usize >= self.width {
                    break;
                }
                let y1 = (k * x as f32 + b) as usize;
                let y2 = (k * (x + 1) as f32 + b) as usize;
                let y_min = y1.min(y2);
                let y_max = y1.max(y2);
                for y in y_min..=y_max {
                    if y >= self.height {
                        break;
                    }
                    *self.pixel_mut(x as usize, y) = color;
                }
            }
        } else {
            // Vertical line
            for y in y_min..y_max {
                if y as usize >= self.height {
                    break;
                }
                *self.pixel_mut(p1.x as usize, y as usize) = color;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    /// AABBGGRR
    pub fn to_u32(&self) -> u32 {
        (self.a as u32) << 0x18 | (self.b as u32) << 0x10 | (self.g as u32) << 0x8 | (self.r as u32)
    }
}

impl From<u32> for Pixel {
    fn from(value: u32) -> Pixel {
        Pixel {
            r: (value & 0x000000FF) as u8,
            g: ((value & 0x0000FF00) >> 0x8) as u8,
            b: ((value & 0x00FF0000) >> 0x10) as u8,
            a: ((value & 0xFF000000) >> 0x18) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel() {
        let raw = 0x44332211;
        let pixel = Pixel::from(raw);
        assert_eq!(pixel.r(), 0x11);
        assert_eq!(pixel.g(), 0x22);
        assert_eq!(pixel.b(), 0x33);
        assert_eq!(pixel.a(), 0x44);
        assert_eq!(pixel.to_u32(), 0x44332211);
    }
}
