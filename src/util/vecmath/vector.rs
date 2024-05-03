use crate::*;

use self::util::float::Num;
use super::{Tuple2, Tuple3};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Tuple3<T> for Vector3<T>
where
    T: Num,
{
    #[inline]
    fn x(&self) -> T {
        self.x
    }

    #[inline]
    fn y(&self) -> T {
        self.y
    }

    #[inline]
    fn z(&self) -> T {
        self.z
    }

    #[inline]
    fn new(x: T, y: T, z: T) -> Self {
        let v = Self { x, y, z };
        debug_assert!(!v.has_nan());
        v
    }
}

super::tuple3_binary!(Vector3, Add, add);
super::tuple3_binary!(Vector3, Sub, sub);
super::scalar_binary!(Vector3, Mul, mul);
super::scalar_binary!(Vector3, Div, div);

impl<T, V> std::ops::Mul<Vector3<T>> for Float
where
    T: std::ops::Mul<Float, Output = V>,
    V: Num,
{
    type Output = Vector3<V>;
    #[inline]
    fn mul(self, rhs: Vector3<T>) -> Vector3<V> {
        rhs * self
    }
}
