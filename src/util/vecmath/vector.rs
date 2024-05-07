use crate::{util::float::Num, Float};

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

impl<T, V> Mul<Vector3<T>> for Float
where
    T: Mul<Float, Output = V>,
    V: Num,
{
    type Output = Vector3<V>;
    #[inline]
    fn mul(self, rhs: Vector3<T>) -> Vector3<V> {
        rhs * self
    }
}

#[inline]
pub fn length_squared<T>(v: Vector3<T>) -> T
where
    T: Num + Add<Output = T>,
{
    v.x().sqr() + v.y().sqr() + v.z.sqr()
}

#[inline]
pub fn length<T>(v: Vector3<T>) -> Float
where
    T: Num + Add<Output = T>,
{
    length_squared(v).sqrt()
}

#[inline]
pub fn normalize<T>(v: Vector3<T>) -> Vector3<T>
where
    T: Num + Add<Output = T>,
    Vector3<T>: Div<Float, Output = Vector3<T>>,
{
    v / length(v)
}

#[inline]
pub fn cross<T>(v: Vector3<T>, w: Vector3<T>) -> Vector3<T>
where
    T: Num,
{
    debug_assert!(!v.has_nan() && !w.has_nan());
    Vector3::<T>::new(
        T::difference_of_products(v.y(), w.z(), v.z(), w.y()),
        T::difference_of_products(v.z(), w.x(), v.x(), w.z()),
        T::difference_of_products(v.x(), w.y(), v.y(), w.x()),
    )
}
