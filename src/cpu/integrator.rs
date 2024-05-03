pub trait Integrator {
    fn render(&self);
}

pub struct PathTracer {}
pub struct RandomWalkIntegrator {}

impl Integrator for RandomWalkIntegrator {
    fn render(&self) {}
}

impl Integrator for PathTracer {
    fn render(&self) {}
}

impl RandomWalkIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}
