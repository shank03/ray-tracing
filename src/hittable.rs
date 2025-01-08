use std::ops::Range;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::empty(),
            normal: Vec3::empty(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, out_normal: &Vec3) {
        self.front_face = r.direction().dot(out_normal) < 0.0;
        self.normal = if self.front_face {
            Vec3::new([*out_normal.x(), *out_normal.y(), *out_normal.z()])
        } else {
            -out_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> Option<Material>;
}

pub struct HittableList<H> {
    objects: Vec<H>,
}
impl<H> HittableList<H>
where
    H: Hittable,
{
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: H) {
        self.objects.push(obj);
    }
}
impl<H> Hittable for HittableList<H>
where
    H: Hittable,
{
    fn hit(&self, r: &Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> Option<Material> {
        let mut temp_rec = HitRecord::new();
        let mut closest = ray_t.end;
        let mut mat = None;

        for obj in self.objects.iter() {
            if let Some(m) = obj.hit(r, ray_t.start..closest, &mut temp_rec) {
                mat = Some(m);
                *rec = temp_rec.clone();
                closest = temp_rec.t;
            }
        }
        mat
    }
}
