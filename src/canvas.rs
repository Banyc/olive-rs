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

    pub fn pixel_over_by(&mut self, x: usize, y: usize, color: Pixel) {
        let p = self.pixel_mut(x, y);
        *p = color.over(*p);
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
        for y in y_min..=y_max {
            if y >= self.height {
                break;
            }
            for x in x_min..=x_max {
                if x >= self.width {
                    break;
                }
                self.pixel_over_by(x, y, color);
            }
        }
    }

    pub fn fill_circle(&mut self, c: Point, r: isize, color: Pixel) {
        let x1 = c.x - r;
        let x2 = c.x + r;
        let y1 = c.y - r;
        let y2 = c.y + r;
        let x_min = x1.min(x2).max(0) as usize;
        let x_max = x1.max(x2).max(0) as usize;
        let y_min = y1.min(y2).max(0) as usize;
        let y_max = y1.max(y2).max(0) as usize;
        for y in y_min..=y_max {
            if y >= self.height {
                break;
            }
            for x in x_min..=x_max {
                if x >= self.width {
                    break;
                }
                let dx = isize::abs_diff(x as isize, c.x);
                let dy = isize::abs_diff(y as isize, c.y);
                if dx * dx + dy * dy <= (r * r) as usize {
                    self.pixel_over_by(x, y, color);
                }
            }
        }
    }

    pub fn draw_line(&mut self, p1: Point, p2: Point, color: Pixel) {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        let y_min = p1.y.min(p2.y).max(0) as usize;
        let y_max = p1.y.max(p2.y).max(0) as usize;
        let x_min = p1.x.min(p2.x).max(0) as usize;
        let x_max = p1.x.max(p2.x).max(0) as usize;

        if dx != 0 {
            let k = dy as f64 / dx as f64;
            let b = p1.y as f64 - k * p1.x as f64;
            for x in x_min..x_max {
                if x >= self.width {
                    break;
                }
                let y1 = (k * x as f64 + b) as usize;
                let y2 = (k * (x + 1) as f64 + b) as usize;
                let y_min = y1.min(y2);
                let y_max = y1.max(y2);
                for y in y_min..=y_max {
                    if y >= self.height {
                        break;
                    }
                    self.pixel_over_by(x, y, color);
                }
            }
        } else {
            // Vertical line
            for y in y_min..=y_max {
                if y >= self.height {
                    break;
                }
                self.pixel_over_by(p1.x as usize, y, color);
            }
        }
    }

    pub fn fill_triangle(&mut self, v1: Point, v2: Point, v3: Point, color: Pixel) {
        let x_min = v1.x.min(v2.x).min(v3.x).max(0) as usize;
        let x_max = v1.x.max(v2.x).max(v3.x).max(0) as usize;
        let y_min = v1.y.min(v2.y).min(v3.y).max(0) as usize;
        let y_max = v1.y.max(v2.y).max(v3.y).max(0) as usize;
        for x in x_min..=x_max {
            if x >= self.width {
                break;
            }
            for y in y_min..=y_max {
                if y >= self.height {
                    break;
                }
                let p = Point {
                    x: x as isize,
                    y: y as isize,
                };
                if is_inside_triangle(p, v1, v2, v3) {
                    self.pixel_over_by(x, y, color);
                }
            }
        }
    }
}

/// - Ref:
///   - implementation: <https://stackoverflow.com/a/2049593/9920172>
///   - Determinant: <https://www.khanacademy.org/math/precalculus/x9e81a4f98389efdf:matrices/x9e81a4f98389efdf:matrices-as-transformations/v/interpreting-determinants-in-terms-of-area>
fn is_inside_triangle(p: Point, v1: Point, v2: Point, v3: Point) -> bool {
    /// Determinant of the following matrix:
    ///
    /// ```text
    /// (p1.x - p0.x) & (p2.x - p0.x) \\
    /// (p1.y - p0.y) & (p2.y - p0.y) \\
    /// ```
    ///
    /// - $(p1.x - p0.x, p1.y - p0.y)^T$:
    ///   - a vector starts from p0 to p1
    ///   - say, the vector is v1
    /// - $(p2.x - p0.x, p2.y - p0.y)^T$:
    ///   - a vector starts from p0 to p2
    ///   - say, the vector is v2
    /// - Before transformation, v2 is on the left side of v1
    /// - The determinant is positive as long as v2 is still on the left side of v1
    /// - The determinant is negative as long as v2 is on the right side of v1
    /// - The determinant is zero when v2 is on the same line as v1
    fn determinant(p0: Point, p1: Point, p2: Point) -> isize {
        (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y)
    }
    let d1 = determinant(p, v1, v2);
    let d2 = determinant(p, v2, v3);
    let d3 = determinant(p, v3, v1);
    let all_neg = d1 < 0 && d2 < 0 && d3 < 0;
    let all_pos = d1 > 0 && d2 > 0 && d3 > 0;
    let any_zero = d1 == 0 || d2 == 0 || d3 == 0;

    // Either:
    // - all:
    //   - vector $v1 - p$ is on the left side of $v2 - p$
    //   - vector $v2 - p$ is on the left side of $v3 - p$
    //   - vector $v3 - p$ is on the left side of $v1 - p$
    // - all:
    //   - vector $v1 - p$ is on the right side of $v2 - p$
    //   - vector $v2 - p$ is on the right side of $v3 - p$
    //   - vector $v3 - p$ is on the right side of $v1 - p$
    // - any:
    //   - vector $v1 - p$ is on the same line as $v2 - p$
    //   - vector $v2 - p$ is on the same line as $v3 - p$
    //   - vector $v3 - p$ is on the same line as $v1 - p$
    all_neg || all_pos || any_zero
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
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

    #[must_use]
    pub fn over(&self, other: Pixel) -> Pixel {
        /// - `c1` over `c2`
        /// - Ref: <https://en.wikipedia.org/wiki/Alpha_compositing>
        fn mix_colors(c1: Pixel, c2: Pixel) -> Pixel {
            let a1 = c1.a as f64 / u8::MAX as f64;
            let a2 = c2.a as f64 / u8::MAX as f64;
            let a0 = a1 + (1. - a1) * a2;
            let r = mix_comps(c1.r, c2.r, a0, a1, a2);
            let g = mix_comps(c1.g, c2.g, a0, a1, a2);
            let b = mix_comps(c1.b, c2.b, a0, a1, a2);
            let a = mix_comps(c1.a, c2.a, a0, a1, a2);
            Pixel::new(r, g, b, a)
        }

        /// `c1` over `c2`
        fn mix_comps(c1: u8, c2: u8, a0: f64, a1: f64, a2: f64) -> u8 {
            if a0 == 0. {
                return 0;
            }
            let c1 = c1 as f64;
            let c2 = c2 as f64;
            let c0 = (c1 * a1 + (1. - a1) * a2 * c2) / a0;
            c0 as u8
        }

        mix_colors(*self, other)
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
