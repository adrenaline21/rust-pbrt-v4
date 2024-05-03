use crate::Float;

pub const PI: Float = std::f64::consts::PI as Float;

#[inline]
pub fn lerp(x: Float, a: Float, b: Float) -> Float {
    (1.0 - x) * a + x * b
}

#[inline]
pub fn fma(x: Float, y: Float, z: Float) -> Float {
    x.mul_add(y, z)
}

mod compensated_float;
mod interval;
