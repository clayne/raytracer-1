use crate::vec::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
