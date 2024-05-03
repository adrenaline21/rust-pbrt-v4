use crate::Float;

use std::ops::{Add, AddAssign};

struct Interval {
    low: Float,
    high: Float,
}

impl Interval {}

impl Add<Interval> for Interval {
    type Output = Interval;
    fn add(self, rhs: Self) -> Self::Output {
        Interval {
            low: self.low + rhs.low,
            high: self.high + rhs.high,
        }
    }
}

mod test {}
