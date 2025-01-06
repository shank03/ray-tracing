#pragma once

#include "color.h"
#include "hittable.h"
#include "vec3.h"

class material {
public:
    virtual ~material() = default;

    virtual bool scatter(const ray &r_in, const hit_record &rec, color &attenuation, ray &scattered) const {
        return false;
    }
};

class lambertian : public material {
public:
    lambertian(const color &albedo) : albedo(albedo) {}

    bool scatter(const ray &r_in, const hit_record &rec, color &attenuation, ray &scattered) const override {
        auto scatter_dir = rec.normal + random_unit_vector();

        if (scatter_dir.near_zero()) {
            scatter_dir = rec.normal;
        }

        scattered   = ray(rec.p, scatter_dir);
        attenuation = albedo;
        return true;
    }

private:
    color albedo;
};

class metal : public material {
public:
    metal(const color &albedo) : albedo(albedo) {}

    metal(const color &albedo, double fuzz) : albedo(albedo), fuzz(fuzz < 1 ? fuzz : 1) {}

    bool scatter(const ray &r_in, const hit_record &rec, color &attenuation, ray &scattered) const override {
        vec3 reflected = reflect(r_in.direction(), rec.normal);
        reflected      = unit_vector(reflected) + (fuzz * random_unit_vector());
        scattered      = ray(rec.p, reflected);
        attenuation    = albedo;
        return (dot(scattered.direction(), rec.normal) > 0);
    }

private:
    color  albedo;
    double fuzz;
};

class dielectric : public material {
public:
    dielectric(double refraction_index) : refraction_index(refraction_index) {}

    bool scatter(const ray &r_in, const hit_record &rec, color &attenuation, ray &scattered) const override {
        attenuation = color(1.0, 1.0, 1.0);
        double ri = rec.front_face ? (1.0/refraction_index) : refraction_index;

        vec3 unit_dir = unit_vector(r_in.direction());
        vec3 refracted = refract(unit_dir, rec.normal, ri);

        scattered = ray(rec.p, refracted);
        return true;
    }

private:
    double refraction_index;
};
