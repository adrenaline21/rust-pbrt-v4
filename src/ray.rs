use crate::util::vecmath::*;
use crate::Float;

// No time
pub struct Ray {
    o: Point3f,
    d: Vector3f,
}

impl Ray {
    fn new(o: Point3f, d: Vector3f) -> Self {
        Ray { o, d }
    }
    fn at(&self, t: Float) -> Point3f {
        self.o + t * self.d
    }
}
mod test {
    #[test]
    fn ray() {
        use super::*;
        let r = Ray::new(Point3f::new(0.0, 0.0, 0.0), Vector3f::new(1.0, 2.0, 4.0));
        assert_eq!(r.at(1.7), Point3f::new(1.7, 3.4, 6.8));
    }
}
