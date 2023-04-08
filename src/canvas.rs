use std::cmp::Ordering;

pub use self::points::{Point, PointF};

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
        if w == 0 || h == 0 {
            return;
        }

        // Trim each edge off one pixel since `p` takes up one pixel.
        let w = trim_edge(w);
        let h = trim_edge(h);

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
        if r == 0 {
            return;
        }

        // Trim radius off one pixel since `c` takes up one pixel.
        let r = trim_edge(r);

        let x1 = c.x - r;
        let x2 = c.x + r;
        let y1 = c.y - r;
        let y2 = c.y + r;
        let x_min = x1.min(x2).max(0) as usize;
        let x_max = x1.max(x2).max(0) as usize;
        let y_min = y1.min(y2).max(0) as usize;
        let y_max = y1.max(y2).max(0) as usize;

        /// Say we have a line of size 1 and we divided it into `res` segments evenly.
        ///
        /// The middle of the line is at 0.5.
        ///
        /// It returns the offset from the middle point of the original line to the middle point of the `i`th segment.
        fn offset_from_middle(i: usize, res: usize) -> f64 {
            let offset_from_zero = (i + 1) as f64 / (res + 1) as f64;
            offset_from_zero - 1. / 2.
        }

        const RESOLUTION: usize = 2;

        fn is_inside_circle(dx: f64, dy: f64, r: f64) -> bool {
            dx * dx + dy * dy <= (r * r)
        }

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
                let mut sub_pixels_filled = 0;
                for sub_y_i in 0..RESOLUTION {
                    let dy = dy as f64 + offset_from_middle(sub_y_i, RESOLUTION);
                    for sub_x_i in 0..RESOLUTION {
                        let dx = dx as f64 + offset_from_middle(sub_x_i, RESOLUTION);
                        if is_inside_circle(dx, dy, r as f64) {
                            sub_pixels_filled += 1;
                        }
                    }
                }
                let sub_pixels_total = RESOLUTION * RESOLUTION;
                let sub_pixels_filled = sub_pixels_filled as f64 / sub_pixels_total as f64;
                let alpha = color.a() as f64 * sub_pixels_filled;
                let color = Pixel::new(color.r(), color.g(), color.b(), alpha as u8);
                self.pixel_over_by(x, y, color);

                // If we don't want to use sub-pixels, we can use this instead.
                // if dx * dx + dy * dy <= (r * r) as usize {
                //     self.pixel_over_by(x, y, color);
                // }
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
                if is_inside_triangle(p.into(), v1.into(), v2.into(), v3.into()) {
                    self.pixel_over_by(x, y, color);
                }
            }
        }
    }
}

/// - Ref:
///   - implementation: <https://stackoverflow.com/a/2049593/9920172>
///   - Determinant: <https://www.khanacademy.org/math/precalculus/x9e81a4f98389efdf:matrices/x9e81a4f98389efdf:matrices-as-transformations/v/interpreting-determinants-in-terms-of-area>
fn is_inside_triangle(p: PointF, v1: PointF, v2: PointF, v3: PointF) -> bool {
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
    fn determinant(p0: PointF, p1: PointF, p2: PointF) -> f64 {
        let (x01, y01) = p0.vector_to(p1);
        let (x02, y02) = p0.vector_to(p2);
        x01 * y02 - x02 * y01
    }
    let d1 = determinant(p, v1, v2);
    let d2 = determinant(p, v2, v3);
    let d3 = determinant(p, v3, v1);
    let all_neg = d1 < 0. && d2 < 0. && d3 < 0.;
    let all_pos = d1 > 0. && d2 > 0. && d3 > 0.;
    let any_zero = d1 == 0. || d2 == 0. || d3 == 0.;

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

fn trim_edge(e: isize) -> isize {
    match e.cmp(&0) {
        Ordering::Less => e + 1,
        Ordering::Greater => e - 1,
        Ordering::Equal => panic!("e is 0"),
    }
}

mod points {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Point {
        pub x: isize,
        pub y: isize,
    }

    impl From<PointF> for Point {
        fn from(p: PointF) -> Self {
            Self {
                x: p.x_round(),
                y: p.y_round(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PointF {
        x: isize,
        x_off: f64,
        y: isize,
        y_off: f64,
    }

    impl PointF {
        pub const fn from_int(x: isize, y: isize) -> Self {
            Self {
                x,
                x_off: 0.,
                y,
                y_off: 0.,
            }
        }

        pub fn from_float(x: isize, x_off: f64, y: isize, y_off: f64) -> Self {
            assert!((0. ..1.).contains(&x_off));
            assert!((0. ..1.).contains(&y_off));
            Self { x, x_off, y, y_off }
        }

        pub fn x_round(&self) -> isize {
            self.x + self.x_off.round() as isize
        }

        pub fn y_round(&self) -> isize {
            self.y + self.y_off.round() as isize
        }

        pub fn vector_to(&self, rhs: Self) -> (f64, f64) {
            let x = rhs.x - self.x;
            let y = rhs.y - self.y;
            let x_off = rhs.x_off - self.x_off;
            let y_off = rhs.y_off - self.y_off;
            (x as f64 + x_off, y as f64 + y_off)
        }
    }

    impl From<Point> for PointF {
        fn from(p: Point) -> Self {
            Self::from_int(p.x, p.y)
        }
    }
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
