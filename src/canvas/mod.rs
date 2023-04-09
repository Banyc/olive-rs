use std::cmp::Ordering;

mod pixel;
mod point;

pub use self::{
    pixel::{HeapPixels2D, Pixel, Pixels2D, StackPixels2D},
    point::{EvenF, Point, PointF},
};

const RESOLUTION: usize = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct Canvas<'pixels, P> {
    pixels2d: &'pixels mut P,
}

impl<'pixels, P> Canvas<'pixels, P>
where
    P: Pixels2D,
{
    pub fn new(pixels2d: &'pixels mut P) -> Self {
        Self { pixels2d }
    }

    pub fn inner(&self) -> &P {
        self.pixels2d
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        assert!(x < self.pixels2d.width());
        assert!(y < self.pixels2d.height());
        let w = self.pixels2d.width();
        &mut self.pixels2d.pixels_mut()[y * w + x]
    }

    pub fn pixel_over_by(&mut self, x: usize, y: usize, color: Pixel) {
        let p = self.pixel_mut(x, y);
        *p = color.over(*p);
    }

    pub fn fill(&mut self, pixel: Pixel) {
        for p in self.pixels2d.pixels_mut() {
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
            if y >= self.pixels2d.height() {
                break;
            }
            for x in x_min..=x_max {
                if x >= self.pixels2d.width() {
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
            if y >= self.pixels2d.height() {
                break;
            }
            for x in x_min..=x_max {
                if x >= self.pixels2d.width() {
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
                if x >= self.pixels2d.width() {
                    break;
                }
                let y1 = (k * x as f64 + b) as usize;
                let y2 = (k * (x + 1) as f64 + b) as usize;
                let y_min = y1.min(y2);
                let y_max = y1.max(y2);
                for y in y_min..=y_max {
                    if y >= self.pixels2d.height() {
                        break;
                    }
                    self.pixel_over_by(x, y, color);
                }
            }
        } else {
            // Vertical line
            for y in y_min..=y_max {
                if y >= self.pixels2d.height() {
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
            if y >= self.pixels2d.height() {
                break;
            }
            for x in x_min..=x_max {
                if x >= self.pixels2d.width() {
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
