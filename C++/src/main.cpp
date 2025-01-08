#include <memory>

#include "_util.h"
#include "camera.h"
#include "hittable_list.h"
#include "material.h"
#include "sphere.h"

int main() {
    // World
    hittable_list world;

    auto ground_material = std::make_shared<lambertian>(color(0.5, 0.5, 0.5));
    world.add(std::make_shared<sphere>(point3(0, -1000, 0), 1000, ground_material));

    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            auto   choose_mat = random_double();
            point3 center(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());

            if ((center - point3(4, 0.2, 0)).length() > 0.9) {
                std::shared_ptr<material> sphere_mat;

                if (choose_mat < 0.8) {
                    // diffuse
                    auto albedo = color::random() * color::random();
                    sphere_mat  = std::make_shared<lambertian>(albedo);
                } else if (choose_mat < 0.95) {
                    auto albedo = color::random(0.5, 1);
                    auto fuzz   = random_double(0, 0.5);
                    sphere_mat  = std::make_shared<metal>(albedo, fuzz);
                } else {
                    sphere_mat = std::make_shared<dielectric>(1.5);
                }
                world.add(std::make_shared<sphere>(center, 0.2, sphere_mat));
            }
        }
    }

    auto material1 = std::make_shared<dielectric>(1.5);
    world.add(std::make_shared<sphere>(point3(0, 1, 0), 1.0, material1));

    auto material2 = std::make_shared<lambertian>(color(0.4, 0.2, 0.1));
    world.add(std::make_shared<sphere>(point3(-4, 1, 0), 1.0, material2));

    auto material3 = std::make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
    world.add(std::make_shared<sphere>(point3(4, 1, 0), 1.0, material3));

    camera cam;

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 1200;
    cam.samples_pp   = 50;
    cam.max_depth    = 10;

    cam.vfov      = 20;
    cam.near_clip = point3(13, 2, 3);
    cam.far_clip  = point3(0, 0, 0);
    cam.vup       = vec3(0, 1, 0);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.render(world);

    return 0;
}
