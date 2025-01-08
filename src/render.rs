use std::{
    f64::INFINITY,
    fs::File,
    io::Write,
    sync::{mpsc, Arc},
    thread,
};

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

    pub fn render(render: Render, world: impl Hittable + 'static) {
        let path = "image.ppm";
        let mut file = File::create(path).expect("Failed to create image file");

        file.write(format!("P3\n{} {}\n255\n", render.image_width, render.image_height).as_bytes())
            .expect("Failed to write image header");

        let mut count = 0usize;
        let pixels_coords = (0..render.image_height)
            .into_iter()
            .fold(Vec::new(), |mut acc, j| {
                (0..render.image_width).into_iter().for_each(|i| {
                    acc.push((count, i as f64, j as f64));
                    count += 1;
                });
                acc
            });

        let start = std::time::Instant::now();

        let cores = thread::available_parallelism().unwrap().get();
        let chunk_size = pixels_coords.len() / cores;

        let mut pixels = vec![[0, 0, 0]; pixels_coords.len()];

        let mut handles = Vec::new();
        let (send_data, recv_data) = mpsc::channel::<(usize, [u8; 3])>();

        let render = Arc::new(render);
        let world = Arc::new(world);
        let send_data = Arc::new(send_data);

        let mut chunks = pixels_coords.chunks(chunk_size).enumerate();
        while let Some((ix, chunk)) = chunks.next() {
            let chunk = chunk.to_vec();

            let render = render.clone();
            let world = world.clone();
            let send_data = send_data.clone();
            handles.push(thread::spawn(move || {
                println!("Started chunk: {ix} - {}", chunk.len());
                for (idx, i, j) in chunk.into_iter() {
                    let mut pixel_color = Color::empty();
                    for _sample in 0..render.samples_per_pixel {
                        let r = render.get_ray(i, j);
                        pixel_color += render.ray_color(r, render.max_depth, world.as_ref());
                    }

                    let pixel = color::get_pixel(render.pixel_sample_scale * &pixel_color);

                    send_data.send((idx, pixel)).unwrap();
                }

                println!("Chunk {ix} completed");
            }));
        }

        for h in handles.into_iter() {
            h.join().unwrap();
        }

        drop(send_data);
        while let Ok((idx, pixel)) = recv_data.recv() {
            pixels[idx] = pixel;
        }

        println!("Elapsed: {:?}", start.elapsed());

        pixels.into_iter().for_each(|[r, g, b]| {
            file.write(format!("{r} {g} {b}\n").as_bytes())
                .expect("Failed to write pixel to image file");
        });

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

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
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
