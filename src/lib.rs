mod camera;
pub mod cpu;
pub mod paramdict;
pub mod parser;
pub mod pbrt;
mod ray;
pub mod scene;
mod util;

pub type Float = f32;
use crate::util::float::Num;
use crate::util::vecmath::Point2f;
use crate::util::vecmath::Vector3f;
