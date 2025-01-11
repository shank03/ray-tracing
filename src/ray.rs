use crate::vec3::{self, Point3, Vec3, VecOp};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
impl Ray {
    pub fn empty() -> Self {
        Self {
            orig: vec3::empty(),
            dir: vec3::empty(),
        }
    }

    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.add(&self.dir.mul_f(t))
    }
}
