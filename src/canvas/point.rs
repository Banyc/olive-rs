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
