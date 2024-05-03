use crate::Float;

pub trait Num: Copy {
    fn is_nan(&self) -> bool;
    fn abs(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

impl Num for Float {
    #[inline]
    fn is_nan(&self) -> bool {
        <Float>::is_nan(*self)
    }
    #[inline]
    fn abs(self) -> Self {
        <Float>::abs(self)
    }
    #[inline]
    fn ceil(self) -> Self {
        <Float>::ceil(self)
    }
    #[inline]
    fn floor(self) -> Self {
        <Float>::floor(self)
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        <Float>::min(self, other)
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        <Float>::max(self, other)
    }
}
