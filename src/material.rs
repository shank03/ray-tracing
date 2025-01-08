use crate::{color::Color, hittable::HitRecord, ray::Ray, utils, vec3::Vec3};

#[derive(Clone)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(
        self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(albedo) => {
                let scatter_dir = &rec.normal + &Vec3::random_unit_vector();

                *scattered = Ray::new(
                    &rec.p,
                    if scatter_dir.near_zero() {
                        &rec.normal
                    } else {
                        &scatter_dir
                    },
                );
                *attenuation = albedo;
                true
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = r_in.direction().reflect(&rec.normal);
                let reflected = &reflected.unit() + &(fuzz * &Vec3::random_unit_vector());

                *scattered = Ray::new(&rec.p, &reflected);
                *attenuation = albedo;
                scattered.direction().dot(&rec.normal) > 0.0
            }
            Material::Dielectric(refraction_index) => {
                *attenuation = Color::new([1.0, 1.0, 1.0]);
                let ri = if rec.front_face {
                    1.0 / refraction_index
                } else {
                    refraction_index
                };

                let unit_dir = r_in.direction().unit();
                let cosine = (-&unit_dir).dot(&rec.normal).min(1.0);
                let sine = (1.0 - cosine * cosine).sqrt();

                let cannot_refract = ri * sine > 1.0;
                let dir = if cannot_refract || Self::reflectance(cosine, ri) > utils::random_float()
                {
                    unit_dir.reflect(&rec.normal)
                } else {
                    unit_dir.refract(&rec.normal, ri)
                };

                *scattered = Ray::new(&rec.p, &dir);
                true
            }
        }
    }

    fn reflectance(cosine: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
