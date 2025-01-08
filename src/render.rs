use std::{f64::INFINITY, fs::File, io::Write};

use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    ray::Ray,
    utils,
    vec3::{Point3, Vec3},
};

pub struct Render {
    image_width: i32,
    image_height: i32,
    max_depth: i32,
    samples_per_pixel: i32,
    pixel_sample_scale: f64,
    camera_center: Point3,
    pixel00_loc: Point3,
    pixel_del_u: Vec3,
    pixel_del_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}
impl Render {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        near_clip: Point3,
        far_clip: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;
        let h = (utils::degress_to_radians(vfov) / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (&near_clip - &far_clip).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = viewport_width * &u;
        let viewport_v = viewport_height * &-&v;

        let pixel_del_u = &viewport_u / image_width as f64;
        let pixel_del_v = &viewport_v / image_height as f64;

        let camera_center = near_clip;
        let viewport_top_left =
            &(&(&camera_center - &(focus_dist * &w)) - &(&viewport_u / 2.0)) - &(&viewport_v / 2.0);
        let pixel00_loc = &viewport_top_left + &(0.5 * &(&pixel_del_u + &pixel_del_v));

        let defocus_radius = focus_dist * utils::degress_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = &u * defocus_radius;
        let defocus_disk_v = &v * defocus_radius;

        Self {
            image_width,
            image_height,
            max_depth,
            samples_per_pixel,
            pixel_sample_scale,
            camera_center,
            pixel00_loc,
            pixel_del_u,
            pixel_del_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        let path = "image.ppm";
        let mut file = File::create(path).expect("Failed to create image file");

        file.write(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .expect("Failed to write image header");

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} \r", self.image_height - j);
            for i in 0..self.image_width {
                let (i, j) = (i as f64, j as f64);

                let mut pixel_color = Color::empty();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, self.max_depth, world);
                }

                let [r, g, b] = color::get_pixel(self.pixel_sample_scale * &pixel_color);
                file.write(format!("{r} {g} {b}\n").as_bytes())
                    .expect("Failed to write pixel to image");
            }
        }

        println!("\rDone                   ");
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = self.sample_square();
        let (x, y, _) = offset.expand();
        let pixel_sample =
            &(&self.pixel00_loc + &((i + x) * &self.pixel_del_u)) + &((j + y) * &self.pixel_del_v);

        let origin = if self.defocus_angle <= 0.0 {
            &self.camera_center
        } else {
            &self.defocus_disk_sample()
        };
        let dir = &pixel_sample - origin;

        Ray::new(origin, &dir)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new([
            utils::random_float() - 0.5,
            utils::random_float() - 0.5,
            0.0,
        ])
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        let (x, y, _) = p.expand();
        &(&self.camera_center + &(x * &self.defocus_disk_u)) + &(y * &self.defocus_disk_v)
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::empty();
        }

        let mut rec = HitRecord::new();
        if let Some(mat) = world.hit(&r, 0.001..INFINITY, &mut rec) {
            let mut scattered = Ray::empty();
            let mut attenuation = Color::empty();

            if mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return &attenuation * &self.ray_color(scattered, depth - 1, world);
            }
            return Color::empty();
        }

        let unit_dir = r.direction().unit();
        let (_, y, _) = unit_dir.expand();
        let a = 0.5 * (y + 1.0);

        &((1.0 - a) * &Color::new([1.0, 1.0, 1.0])) + &(a * &Color::new([0.5, 0.7, 1.0]))
    }
}
