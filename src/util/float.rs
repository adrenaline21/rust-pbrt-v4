use crate::Float;

use super::math::fma;

pub trait Num: Copy {
    fn is_nan(&self) -> bool;
    fn abs(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn sqr(self) -> Self;
    fn sqrt(self) -> Float;
    fn difference_of_products(a: Self, b: Self, c: Self, d: Self) -> Self;
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

    #[inline]
    fn sqr(self) -> Self {
        self * self
    }

    #[inline]
    fn sqrt(self) -> Float {
        <Float>::sqrt(self)
    }

    #[inline]
    fn difference_of_products(a: Self, b: Self, c: Self, d: Self) -> Self {
        let cd = c * d;
        let sum_of_products = fma(a, b, cd);
        let error = fma(c, d, -cd);
        sum_of_products + error
    }
}
