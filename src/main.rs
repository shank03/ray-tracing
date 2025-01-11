use hittable::HittableList;
use material::Material;
use render::Render;
use sphere::Sphere;
use vec3::VecOp;

mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod render;
mod sphere;
mod utils;
mod vec3;

fn main() {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian([0.5, 0.5, 0.5]);
    world.add(Sphere::new([0.0, -1000.0, 0.0], 1000.0, ground_material));

    let ref_point = [4.0, 0.2, 0.0];
    for a in -12..12 {
        for b in -12..12 {
            let (a, b) = (a as f64, b as f64);

            let choose_mat = utils::random_float();
            let center = [
                a + 0.9 * utils::random_float(),
                0.2,
                b + 0.9 * utils::random_float(),
            ];

            if (center.sub(&ref_point)).length() <= 0.9 {
                continue;
            }

            let sphere_mat = match choose_mat {
                x if x < 0.8 => {
                    let albedo = vec3::random().mul(&vec3::random());
                    Material::Lambertian(albedo)
                }
                x if x < 0.95 => {
                    let albedo = vec3::random_min_max(0.5, 1.0);
                    let fuzz = utils::random_min_max(0.0, 0.5);
                    Material::Metal(albedo, fuzz)
                }
                _ => Material::Dielectric(1.5),
            };

            world.add(Sphere::new(center, 0.2, sphere_mat));
        }
    }

    let material1 = Material::Dielectric(1.5);
    world.add(Sphere::new([0.0, 1.0, 0.0], 1.0, material1));

    let material2 = Material::Lambertian([0.4, 0.2, 0.1]);
    world.add(Sphere::new([-4.0, 1.0, 0.0], 1.0, material2));

    let material3 = Material::Metal([0.7, 0.6, 0.5], 0.0);
    world.add(Sphere::new([4.0, 1.0, 0.0], 1.0, material3));

    let renderer = Render::new(
        16.0 / 9.0,
        1920,
        64,
        32,
        32.0,
        [13.0, 2.0, 3.0],
        vec3::empty(),
        [0.0, 1.0, 0.0],
        0.6,
        10.0,
    );

    Render::render(renderer, world);
}
