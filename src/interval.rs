use std::ops::Range;

pub trait Interval {
    fn surrounds(&self, x: f64) -> bool;
    fn clamp(&self, x: f64) -> f64;
}

impl Interval for Range<f64> {
    fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }

    fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.start, self.end)
    }
}
