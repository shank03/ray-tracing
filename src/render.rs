use std::{
    collections::VecDeque,
    f64::INFINITY,
    fs::File,
    io::Write,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    ray::Ray,
    utils,
    vec3::{self, Point3, Vec3, VecOp},
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

        let w = (near_clip.sub(&far_clip)).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u.mul_f(viewport_width);
        let viewport_v = (v.neg()).mul_f(viewport_height);

        let pixel_del_u = viewport_u.div_f(image_width as f64);
        let pixel_del_v = viewport_v.div_f(image_height as f64);

        let camera_center = near_clip;

        let viewport_top_left = camera_center
            .sub(&w.mul_f(focus_dist))
            .sub(&viewport_u.div_f(2.0))
            .sub(&viewport_v.div_f(2.0));
        let pixel00_loc = viewport_top_left.add(&(pixel_del_u.add(&pixel_del_v)).mul_f(0.5));

        let defocus_radius = focus_dist * utils::degress_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u.mul_f(defocus_radius);
        let defocus_disk_v = v.mul_f(defocus_radius);

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
        let chunk_size = u16::MAX as usize;

        let mut pixels = vec![[0, 0, 0]; pixels_coords.len()];

        let mut handles = Vec::new();
        let (send_data, recv_data) = mpsc::channel::<(usize, [u8; 3])>();

        let render = Arc::new(render);
        let world = Arc::new(world);
        let send_data = Arc::new(send_data);

        let chunks = pixels_coords
            .chunks(chunk_size)
            .enumerate()
            .map(|(i, c)| (i, c.to_vec()))
            .collect::<Vec<_>>();
        drop(pixels_coords);

        let mut jobs: VecDeque<Box<dyn FnOnce() + Send + 'static>> = VecDeque::new();
        for (ix, chunk) in chunks.into_iter() {
            let render = render.clone();
            let world = world.clone();
            let send_data = send_data.clone();

            jobs.push_back(Box::new(move || {
                println!("Started chunk: {ix} - {:?}", thread::current().id());
                for (idx, i, j) in chunk.into_iter() {
                    let mut pixel_color = vec3::empty();
                    for _sample in 0..render.samples_per_pixel {
                        let r = render.get_ray(i, j);
                        pixel_color.add_assign(render.ray_color(
                            r,
                            render.max_depth,
                            world.as_ref(),
                        ));
                    }

                    let pixel = color::get_pixel(pixel_color.mul_f(render.pixel_sample_scale));
                    send_data.send((idx, pixel)).unwrap();
                }

                println!("Chunk {ix} completed");
            }));
        }

        let jobs = Arc::new(Mutex::new(jobs));
        for _ in 0..cores {
            let jobs = jobs.clone();
            handles.push(thread::spawn(move || loop {
                let job = {
                    let mut jobs = jobs.lock().unwrap();
                    if jobs.is_empty() {
                        break;
                    }
                    jobs.pop_front()
                };

                if let Some(job) = job {
                    job();
                }
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
        let (x, y) = (utils::random_float() - 0.5, utils::random_float() - 0.5);
        let pixel_sample = self
            .pixel00_loc
            .add(&self.pixel_del_u.mul_f(i + x))
            .add(&self.pixel_del_v.mul_f(j + y));

        let origin = if self.defocus_angle <= 0.0 {
            &self.camera_center
        } else {
            &self.defocus_disk_sample()
        };
        let dir = pixel_sample.sub(origin);

        Ray::new(origin, &dir)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let [x, y, _] = vec3::random_in_unit_disk();
        self.camera_center
            .add(&self.defocus_disk_u.mul_f(x))
            .add(&self.defocus_disk_v.mul_f(y))
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return vec3::empty();
        }

        let mut rec = HitRecord::new();
        if let Some(mat) = world.hit(&r, 0.001..INFINITY, &mut rec) {
            let mut scattered = Ray::empty();
            let mut attenuation = vec3::empty();

            if mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation.mul(&self.ray_color(scattered, depth - 1, world));
            }
            return vec3::empty();
        }

        let [_, y, _] = r.direction().unit();
        let a = 0.5 * (y + 1.0);

        [1.0, 1.0, 1.0]
            .mul_f(1.0 - a)
            .add(&[0.5, 0.7, 1.0].mul_f(a))
    }
}
