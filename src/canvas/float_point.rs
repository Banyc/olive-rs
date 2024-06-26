use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct FloatSpace {
    x_axis_range: RangeInclusive<f64>,
    y_axis_range: RangeInclusive<f64>,
}

impl FloatSpace {
    pub const fn new(x_axis_range: RangeInclusive<f64>, y_axis_range: RangeInclusive<f64>) -> Self {
        Self {
            x_axis_range,
            y_axis_range,
        }
    }

    pub fn x_axis_range(&self) -> &RangeInclusive<f64> {
        &self.x_axis_range
    }

    pub fn y_axis_range(&self) -> &RangeInclusive<f64> {
        &self.y_axis_range
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FloatPoint {
    x: f64,
    y: f64,
}

impl FloatPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}
