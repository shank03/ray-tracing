use color::Color;
use hittable::HittableList;
use material::Material;
use render::Render;
use sphere::Sphere;
use vec3::{Point3, Vec3};

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

    let ground_material = Material::Lambertian(Color::new([0.5, 0.5, 0.5]));
    world.add(Sphere::new(
        Point3::new([0.0, -1000.0, 0.0]),
        1000.0,
        ground_material,
    ));

    let ref_point = Point3::new([4.0, 0.2, 0.0]);
    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);

            let choose_mat = utils::random_float();
            let center = Point3::new([
                a + 0.9 * utils::random_float(),
                0.2,
                b + 0.9 * utils::random_float(),
            ]);

            if (&center - &ref_point).len() <= 0.9 {
                continue;
            }

            let sphere_mat = match choose_mat {
                x if x < 0.8 => {
                    let albedo = &Color::random() * &Color::random();
                    Material::Lambertian(albedo)
                }
                x if x < 0.95 => {
                    let albedo = Color::random_min_max(0.5, 1.0);
                    let fuzz = utils::random_min_max(0.0, 0.5);
                    Material::Metal(albedo, fuzz)
                }
                _ => Material::Dielectric(1.5),
            };

            world.add(Sphere::new(center, 0.2, sphere_mat));
        }
    }

    let material1 = Material::Dielectric(1.5);
    world.add(Sphere::new(Point3::new([0.0, 1.0, 0.0]), 1.0, material1));

    let material2 = Material::Lambertian(Color::new([0.4, 0.2, 0.1]));
    world.add(Sphere::new(Point3::new([-4.0, 1.0, 0.0]), 1.0, material2));

    let material3 = Material::Metal(Color::new([0.7, 0.6, 0.5]), 0.0);
    world.add(Sphere::new(Point3::new([4.0, 1.0, 0.0]), 1.0, material3));

    let renderer = Render::new(
        16.0 / 9.0,
        1920,
        64,
        50,
        20.0,
        Point3::new([13.0, 2.0, 3.0]),
        Point3::empty(),
        Vec3::new([0.0, 1.0, 0.0]),
        0.6,
        10.0,
    );

    Render::render(renderer, world);
}
