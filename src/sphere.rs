use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, VecOp},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Material) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> Option<Material> {
        let oc = self.center.sub(r.origin());

        let a = r.direction().dot(r.direction());
        let b = r.direction().dot(&oc);
        let c = oc.dot(&oc) - (self.radius * self.radius);

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let normal = rec.p.sub(&self.center).div_f(self.radius);
        rec.set_face_normal(r, &normal);

        Some(self.mat.clone())
    }
}
