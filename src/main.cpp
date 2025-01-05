#include <iostream>

#include "color.h"
#include "ray.h"
#include "vec3.h"

color ray_color(const ray &r) {
    vec3 unit_dir = unit_vector(r.direction());
    auto a        = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0);
}

int main() {
    auto aspect_ratio = 16.0 / 9.0;
    int  image_width  = 400;

    // make sure image height is at least 1
    int image_height = int(image_width / aspect_ratio);
    image_height     = (image_height < 1) ? 1 : image_height;

    // Camera
    auto focal_length  = 1.0;
    auto camera_center = point3(0, 0, 0);

    // Viewport width is fine being less than 1
    auto viewport_height = 2.0;
    auto viewport_width  = viewport_height * (double(image_width) / image_height);

    // horizontal and vertical vectors, where top left = (u,v)=(0,0)
    auto viewport_u = vec3(viewport_width, 0, 0);
    auto viewport_v = vec3(0, -viewport_height, 0);

    // viewport to image delta
    auto pixel_del_u = viewport_u / image_width;
    auto pixel_del_v = viewport_v / image_height;

    auto viewport_top_left = camera_center - vec3(0, 0, focal_length) - viewport_u / 2 - viewport_v / 2;
    auto pixel00_loc       = viewport_top_left + 0.5 * (pixel_del_u + pixel_del_v);

    std::cout << "P3\n"
              << image_width << ' ' << image_height << "\n255\n";

    for (int j = 0; j < image_height; j++) {
        std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
        for (int i = 0; i < image_width; i++) {
            auto pixel_center = pixel00_loc + (i * pixel_del_u) + (j * pixel_del_v);
            auto ray_dir      = pixel_center - camera_center;
            ray  r(camera_center, ray_dir);

            color pixel = ray_color(r);
            write_color(std::cout, pixel);
        }
    }

    std::clog << "\rDone.                 \n";

    return 0;
}
