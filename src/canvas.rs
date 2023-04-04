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

    pub fn fill_rect(&mut self, x0: isize, y0: isize, w: isize, h: isize, color: Pixel) {
        let y_min = y0.min(y0 + h).max(0) as usize;
        let y_max = y0.max(y0 + h).max(0) as usize;
        let x_min = x0.min(x0 + w).max(0) as usize;
        let x_max = x0.max(x0 + w).max(0) as usize;
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

    pub fn fill_circle(&mut self, cx: isize, cy: isize, r: isize, color: Pixel) {
        let x1 = cx - r;
        let x2 = cx + r;
        let y1 = cy - r;
        let y2 = cy + r;
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
                let dx = isize::abs_diff(x, cx);
                let dy = isize::abs_diff(y, cy);
                if dx * dx + dy * dy <= (r * r) as usize {
                    *self.pixel_mut(x as usize, y as usize) = color;
                }
            }
        }
    }

    pub fn draw_line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: Pixel) {
        let dx = x2 - x1;
        let dy = y2 - y1;

        let y_min = y1.min(y2).max(0);
        let y_max = y1.max(y2).max(0);
        let x_min = x1.min(x2).max(0);
        let x_max = x1.max(x2).max(0);

        if dx != 0 {
            let k = dy as f32 / dx as f32;
            let b = y1 as f32 - k * x1 as f32;
            for x in x_min..x_max {
                if x as usize >= self.width {
                    break;
                }
                let ty1 = (k * x as f32 + b) as usize;
                let ty2 = (k * (x + 1) as f32 + b) as usize;
                let y_min = ty1.min(ty2);
                let y_max = ty1.max(ty2);
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
                *self.pixel_mut(x1 as usize, y as usize) = color;
            }
        }
    }
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
