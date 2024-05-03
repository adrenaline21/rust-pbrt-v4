use super::ray::*;
use crate::*;

trait Camera {
    fn generate_ray() -> Option<CameraRay>;
}

struct CameraSample {
    p_film: Point2f,
}

struct CameraRay {
    ray: Ray,
    // weight: SampledSpectrum,
}
