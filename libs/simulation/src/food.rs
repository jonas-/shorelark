use crate::*;

#[derive(Debug)]
pub struct Food {
    pub position: na::Point2<f64>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f64> {
        self.position
    }
}
