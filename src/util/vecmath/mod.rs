use crate::util::float::Num;
use crate::*;

mod normal;
mod point;
pub mod vector;

pub type Vector3f = vector::Vector3<Float>;
pub type Point3f = point::Point3<Float>;
pub type Point2f = point::Point2<Float>;

pub trait Tuple2<T: Num> {
    fn new(x: T, y: T) -> Self;

    fn x(&self) -> T;
    fn y(&self) -> T;

    #[inline]
    fn has_nan(&self) -> bool {
        self.x().is_nan() || self.y().is_nan()
    }

    #[inline]
    fn abs(self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().abs(), self.y().abs())
    }

    #[inline]
    fn ceil(self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().ceil(), self.y().ceil())
    }

    #[inline]
    fn floor(self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().floor(), self.y().floor())
    }

    #[inline]
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().min(other.x()), self.y().min(other.y()))
    }

    #[inline]
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().max(other.x()), self.y().max(other.y()))
    }
}

macro_rules! tuple2_binary {
    ($name:ident, $Op:ident, $op:ident) => {
        impl<T, U, V> $Op<$name<U>> for $name<T>
        where
            T: $Op<U, Output = V>,
            U: Num,
            V: Num,
        {
            type Output = $name<V>;
            #[inline]
            fn $op(self, rhs: $name<U>) -> Self::Output {
                debug_assert!(!rhs.has_nan());
                Self::Output::new(self.x.$op(rhs.x), self.y.$op(rhs.y))
            }
        }
    };
}
use tuple2_binary;

pub trait Tuple3<T: Num> {
    fn new(x: T, y: T, z: T) -> Self;

    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;

    #[inline]
    fn has_nan(&self) -> bool {
        self.x().is_nan() || self.y().is_nan() || self.z().is_nan()
    }

    #[inline]
    fn abs(self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    #[inline]
    fn ceil(self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().ceil(), self.y().ceil(), self.z().ceil())
    }

    #[inline]
    fn floor(self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x().floor(), self.y().floor(), self.z().floor())
    }

    #[inline]
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(
            self.x().min(other.x()),
            self.y().min(other.y()),
            self.z().min(other.z()),
        )
    }

    #[inline]
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(
            self.x().max(other.x()),
            self.y().max(other.y()),
            self.z().max(other.z()),
        )
    }
}

macro_rules! tuple3_binary {
    ($name:ident, $Op:ident, $op:ident) => {
        impl<T, U, V> $Op<$name<U>> for $name<T>
        where
            T: $Op<U, Output = V>,
            U: Num,
            V: Num,
        {
            type Output = $name<V>;
            #[inline]
            fn $op(self, rhs: $name<U>) -> Self::Output {
                debug_assert!(!rhs.has_nan());
                Self::Output::new(self.x.$op(rhs.x), self.y.$op(rhs.y), self.z.$op(rhs.z))
            }
        }
    };
}
use tuple3_binary;

macro_rules! scalar_binary {
    ($name:ident, $Op:ident, $op:ident) => {
        impl<T, Scalar, V> $Op<Scalar> for $name<T>
        where
            T: $Op<Scalar, Output = V>,
            Scalar: Copy,
            V: Num,
        {
            type Output = $name<V>;
            #[inline]
            fn $op(self, s: Scalar) -> Self::Output {
                Self::Output::new(self.x.$op(s), self.y.$op(s), self.z.$op(s))
            }
        }
    };
}
use scalar_binary;

mod test {

    #[test]
    fn vector3_basics() {
        use super::*;
        use crate::Vector3f;

        let vf = Vector3f::new(-1.0, 10.0, 2.0);
        assert_ne!(vf, Vector3f::new(-1.0, 100.0, 2.0));
        assert_eq!(Vector3f::new(-2.0, 20.0, 4.0), vf + vf);
        assert_eq!(Vector3f::new(0.0, 0.0, 0.0), vf - vf);
        assert_eq!(Vector3f::new(-2.0, 20.0, 4.0), vf * 2.0);
        assert_eq!(Vector3f::new(-2.0, 20.0, 4.0), 2.0 * vf);
        assert_eq!(Vector3f::new(-0.5, 5.0, 1.0), vf / 2.0);
        assert_eq!(Vector3f::new(1.0, 10.0, 2.0), vf.abs());
        assert_eq!(vf, Vector3f::new(-1.5, 9.9, 1.01).ceil());
        assert_eq!(vf, Vector3f::new(-0.5, 10.01, 2.99).floor());
        assert_eq!(
            Vector3f::new(-20.0, 10.0, 1.5),
            vf.min(Vector3f::new(-20.0, 20.0, 1.5))
        );
        assert_eq!(
            Vector3f::new(-1.0, 20.0, 2.0),
            vf.max(Vector3f::new(-20.0, 20.0, 0.0))
        );
    }

    #[test]
    fn vector_angle_between() {}
}
