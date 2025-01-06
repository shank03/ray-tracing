#pragma once

#include "_util.h"
#include "color.h"
#include "hittable.h"
#include "interval.h"
#include "material.h"
#include "ray.h"
#include "vec3.h"

class camera {
public:
    double aspect_ratio = 16.0 / 9.0;
    int    image_width  = 1920;
    int    samples_pp   = 10;
    int    max_depth    = 10;

    double vfov      = 90;
    point3 near_clip = point3(0, 0, 0);
    point3 far_clip  = point3(0, 0, -1);
    vec3   vup       = vec3(0, 1, 0);

    double defocus_angle = 0;
    double focus_dist    = 10;

    void render(const hittable &world) {
        init();

        std::cout << "P3\n"
                  << image_width << ' ' << image_height << "\n255\n";

        for (int j = 0; j < image_height; j++) {
            std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
            for (int i = 0; i < image_width; i++) {
                color pixel_color(0, 0, 0);
                for (int sample = 0; sample < samples_pp; sample++) {
                    ray r = get_ray(i, j);
                    pixel_color += ray_color(r, max_depth, world);
                }

                write_color(std::cout, pixel_sample_scale * pixel_color);
            }
        }

        std::clog << "\rDone.                 \n";
    }

private:
    int    image_height;
    double pixel_sample_scale;
    point3 camera_center, pixel00_loc;
    vec3   pixel_del_u, pixel_del_v;
    vec3   u, v, w;
    vec3   defocus_disk_u, defocus_disk_v;

    void init() {
        image_height = int(image_width / aspect_ratio);
        image_height = (image_height < 1) ? 1 : image_height;

        pixel_sample_scale = 1.0 / samples_pp;

        // Camera
        camera_center = near_clip;
        auto theta    = degress_to_radians(vfov);
        auto h        = std::tan(theta / 2);

        // Viewport width is fine being less than 1
        auto viewport_height = 2 * h * focus_dist;
        auto viewport_width  = viewport_height * (double(image_width) / image_height);

        w = unit_vector(near_clip - far_clip);
        u = unit_vector(cross(vup, w));
        v = cross(w, u);

        // horizontal and vertical vectors, where top left = (u,v)=(0,0)
        auto viewport_u = viewport_width * u;
        auto viewport_v = viewport_height * -v;

        // viewport to image delta
        pixel_del_u = viewport_u / image_width;
        pixel_del_v = viewport_v / image_height;

        auto viewport_top_left = camera_center - (focus_dist * w) - viewport_u / 2 - viewport_v / 2;
        pixel00_loc            = viewport_top_left + 0.5 * (pixel_del_u + pixel_del_v);

        auto defocus_radius = focus_dist * std::tan(degress_to_radians(defocus_angle / 2));
        defocus_disk_u      = u * defocus_radius;
        defocus_disk_v      = v * defocus_radius;
    }

    ray get_ray(int i, int j) const {
        auto offset       = sample_square();
        auto pixel_sample = pixel00_loc + ((i + offset.x()) * pixel_del_u) + ((j + offset.y()) * pixel_del_v);

        auto origin = (defocus_angle <= 0) ? camera_center : defocus_disk_sample();
        auto dir    = pixel_sample - origin;

        return ray(origin, dir);
    }

    vec3 sample_square() const {
        return vec3(random_double() - 0.5, random_double() - 0.5, 0);
    }

    point3 defocus_disk_sample() const {
        auto p = random_in_unit_disk();
        return camera_center + (p[0] * defocus_disk_u) + (p[1] * defocus_disk_v);
    }

    color ray_color(const ray &r, int depth, const hittable &world) const {
        if (depth <= 0) return color(0, 0, 0);

        hit_record rec;
        if (world.hit(r, interval(0.001, infinity), rec)) {
            ray   scattered;
            color attenuation;
            if (rec.mat->scatter(r, rec, attenuation, scattered)) {
                return attenuation * ray_color(scattered, depth - 1, world);
            }
            return color(0, 0, 0);
        }

        vec3 unit_dir = unit_vector(r.direction());
        auto a        = 0.5 * (unit_dir.y() + 1.0);
        return (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0);
    }
};
