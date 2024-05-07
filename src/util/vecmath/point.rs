use super::vector::*;
use super::*;

use std::ops::{Add, Div, Mul, Sub};

pub struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Tuple2<T> for Point2<T>
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
    fn new(x: T, y: T) -> Self {
        let v = Self { x, y };
        debug_assert!(!v.has_nan());
        v
    }
}
super::tuple2_binary!(Point2, Add, add);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Tuple3<T> for Point3<T>
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

super::tuple3_binary!(Point3, Add, add);
// super::tuple3_binary!(Point3, Sub, sub)
impl<T, U, V> Sub<Point3<U>> for Point3<T>
where
    T: Sub<U, Output = V>,
    U: Num,
    V: Num,
{
    type Output = Vector3<V>;
    #[inline]
    fn sub(self, rhs: Point3<U>) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        Self::Output::new(self.x.sub(rhs.x), self.y.sub(rhs.y), self.z.sub(rhs.z))
    }
}

super::scalar_binary!(Point3, Mul, mul);
super::scalar_binary!(Point3, Div, div);

impl<T, U, V> std::ops::Add<Vector3<T>> for Point3<U>
where
    U: std::ops::Add<T, Output = V>,
    V: Num,
{
    type Output = Point3<V>;
    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
