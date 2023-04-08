use std::cmp::Ordering;

pub use self::points::{EvenF, Point, PointF};

const RESOLUTION: usize = 2;

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

    pub fn fill_circle(&mut self, c: PointF, r: f64, color: Pixel) {
        let x1 = (c.x().add_f(-r)).floor();
        let x2 = (c.x().add_f(r)).ceil();
        let y1 = (c.y().add_f(-r)).floor();
        let y2 = (c.y().add_f(r)).ceil();
        let x_min = x1.min(x2).max(0) as usize;
        let x_max = x1.max(x2).max(0) as usize;
        let y_min = y1.min(y2).max(0) as usize;
        let y_max = y1.max(y2).max(0) as usize;

        fn is_inside_circle(dx: f64, dy: f64, r: f64) -> bool {
            dx * dx + dy * dy <= r * r
        }

        fn is_far_outside_circle(dx: f64, dy: f64, r: f64) -> bool {
            let r1 = if r.is_sign_positive() { r + 1. } else { r - 1. };
            dx * dx + dy * dy >= r1 * r1
        }

        fn is_far_inside_circle(dx: f64, dy: f64, r: f64) -> bool {
            let r1 = if r.is_sign_positive() && r > 1. {
                r - 1.
            } else if r.is_sign_negative() && r < -1. {
                r + 1.
            } else {
                return false;
            };
            dx * dx + dy * dy <= r1 * r1
        }

        for y in y_min..=y_max {
            if y >= self.height {
                break;
            }
            for x in x_min..=x_max {
                if x >= self.width {
                    break;
                }
                let dx = EvenF::new(x as isize, 0.) - c.x();
                let dy = EvenF::new(y as isize, 0.) - c.y();
                let dx = dx.to_f();
                let dy = dy.to_f();

                if is_far_outside_circle(dx, dy, r) {
                    continue;
                }

                if is_far_inside_circle(dx, dy, r) {
                    self.pixel_over_by(x, y, color);
                    continue;
                }

                let mut sub_pixels_filled = 0;
                offset_from_middle_iter().for_each(|y_off| {
                    let dy = dy + y_off;
                    offset_from_middle_iter().for_each(|x_off| {
                        let dx = dx + x_off;
                        if is_inside_circle(dx, dy, r) {
                            sub_pixels_filled += 1;
                        }
                    })
                });
                self.set_anti_aliasing_pixel(x, y, color, sub_pixels_filled);
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

    pub fn fill_triangle(&mut self, v1: PointF, v2: PointF, v3: PointF, color: Pixel) {
        let x_min = v1
            .x()
            .floor()
            .min(v2.x().floor())
            .min(v3.x().floor())
            .max(0) as usize;
        let x_max = v1.x().ceil().max(v2.x().ceil()).max(v3.x().ceil()).max(0) as usize;
        let y_min = v1
            .y()
            .floor()
            .min(v2.y().floor())
            .min(v3.y().floor())
            .max(0) as usize;
        let y_max = v1.y().ceil().max(v2.y().ceil()).max(v3.y().ceil()).max(0) as usize;
        for y in y_min..=y_max {
            if y >= self.height {
                break;
            }
            for x in x_min..=x_max {
                if x >= self.width {
                    break;
                }
                let p = PointF::from_int(x as isize, y as isize);
                if is_inside_triangle(p, v1, v2, v3) {
                    self.pixel_over_by(x, y, color);
                }
            }
        }
    }

    fn set_anti_aliasing_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Pixel,
        sub_pixels_filled: usize,
    ) {
        let sub_pixels_total = RESOLUTION * RESOLUTION;
        let sub_pixels_filled = sub_pixels_filled as f64 / sub_pixels_total as f64;
        let alpha = color.a() as f64 * sub_pixels_filled;
        let color = Pixel::new(color.r(), color.g(), color.b(), alpha as u8);
        self.pixel_over_by(x, y, color);
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
        let (x01, y01) = p0.f_to(p1);
        let (x02, y02) = p0.f_to(p2);
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

/// Say we have a line of size 1 and we divided it into `RESOLUTION` segments evenly.
///
/// The middle of the line is at 0.5.
///
/// There is an offset from the middle point of the original line to the middle point of each segment.
///
/// It iterates through all the offsets.
fn offset_from_middle_iter() -> impl Iterator<Item = f64> {
    /// Say we have a line of size 1 and we divided it into `res` segments evenly.
    ///
    /// The middle of the line is at 0.5.
    ///
    /// It returns the offset from the middle point of the original line to the middle point of the `i`th segment.
    fn offset_from_middle(i: usize, res: usize) -> f64 {
        let offset_from_zero = (i + 1) as f64 / (res + 1) as f64;
        offset_from_zero - 1. / 2.
    }

    (0..RESOLUTION).map(|i| offset_from_middle(i, RESOLUTION))
}

mod points {
    use std::ops::{Add, Sub};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Point {
        pub x: isize,
        pub y: isize,
    }

    impl From<PointF> for Point {
        fn from(p: PointF) -> Self {
            Self {
                x: p.x().round(),
                y: p.y().round(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct EvenF {
        n: isize,
        off: f64, // 0. ..1.
    }

    impl EvenF {
        pub fn new(mut n: isize, mut off: f64) -> Self {
            if !(0. ..1.).contains(&off) {
                let d = off.floor() as isize;
                n += d;
                off -= d as f64;
            }
            assert!((0. ..1.).contains(&off));
            Self { n, off }
        }

        pub fn zero() -> Self {
            Self::new(0, 0.)
        }

        pub fn round(&self) -> isize {
            self.n + self.off.round() as isize
        }

        pub fn floor(&self) -> isize {
            self.n
        }

        pub fn ceil(&self) -> isize {
            self.n + self.off.ceil() as isize
        }

        pub fn to_f(self) -> f64 {
            self.n as f64 + self.off
        }

        pub fn f_to(&self, rhs: Self) -> f64 {
            (rhs.n - self.n) as f64 + rhs.off - self.off
        }

        pub fn add_f(&self, v: f64) -> Self {
            Self::new(self.n, self.off + v)
        }
    }

    impl Add for EvenF {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.n + rhs.n, self.off + rhs.off)
        }
    }

    impl Sub for EvenF {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.n - rhs.n, self.off - rhs.off)
        }
    }

    impl From<f64> for EvenF {
        fn from(v: f64) -> Self {
            Self::new(0, v)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PointF {
        x: EvenF,
        y: EvenF,
    }

    impl PointF {
        pub fn new(x: EvenF, y: EvenF) -> Self {
            Self { x, y }
        }

        pub fn from_int(x: isize, y: isize) -> Self {
            Self {
                x: EvenF::new(x, 0.),
                y: EvenF::new(y, 0.),
            }
        }

        pub fn from_float(x: isize, x_off: f64, y: isize, y_off: f64) -> Self {
            Self {
                x: EvenF::new(x, x_off),
                y: EvenF::new(y, y_off),
            }
        }

        pub fn x(&self) -> EvenF {
            self.x
        }

        pub fn y(&self) -> EvenF {
            self.y
        }

        pub fn f_to(&self, rhs: Self) -> (f64, f64) {
            (self.x.f_to(rhs.x), self.y.f_to(rhs.y))
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
