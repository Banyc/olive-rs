#[derive(Debug, PartialEq, Eq)]
pub struct Pixels<'vec> {
    width: usize,
    height: usize,
    pixels: &'vec mut [Pixel],
}

impl Pixels<'_> {
    pub fn new(width: usize, height: usize, pixels: &mut [Pixel]) -> Pixels {
        assert_eq!(width * height, pixels.len());
        Pixels {
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

    pub fn fill_rect(&mut self, x0: usize, y0: usize, w: usize, h: usize, color: Pixel) {
        for y in y0..(y0 + h) {
            if y >= self.height {
                break;
            }
            for x in x0..(x0 + w) {
                if x >= self.width {
                    break;
                }
                *self.pixel_mut(x, y) = color;
            }
        }
    }

    pub fn fill_circle(&mut self, cx: usize, cy: usize, r: usize, color: Pixel) {
        let x1 = cx.saturating_sub(r);
        let x2 = cx.saturating_add(r);
        let y1 = cy.saturating_sub(r);
        let y2 = cy.saturating_add(r);
        for y in y1..y2 {
            if y >= self.height {
                break;
            }
            for x in x1..x2 {
                if x >= self.width {
                    break;
                }
                let dx = usize::abs_diff(x, cx);
                let dy = usize::abs_diff(y, cy);
                if dx * dx + dy * dy <= r * r {
                    *self.pixel_mut(x, y) = color;
                }
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
