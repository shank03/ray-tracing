#pragma once

#include <memory>

#include "_util.h"
#include "hittable.h"
#include "ray.h"
#include "vec3.h"

class sphere : public hittable {
public:
    sphere(const point3 &center, double radius) : center(center), radius(std::fmax(0, radius)) {
        // TODO:
    }

    sphere(const point3 &center, double radius, std::shared_ptr<material> mat) : center(center), radius(std::fmax(0, radius)), mat(mat) {}

    bool hit(const ray &r, interval ray_t, hit_record &rec) const override {
        // Sphere on arbitary point (Cx, Cy, Cz) => (Cx - x)^2 + (Cy - y)^2 + (Cz - z)^2 = r^2;
        // => vec from point P to center C => (C - P)
        vec3 oc = center - r.origin();

        // => (C - P) * (C - P) = r^2;
        // => at time `t`, P => P(t) => Q + td
        // => (C - (Q + td)) * (C - (Q + td)) = r^2;
        // => (-td + (C-Q)) * (-td + (C-Q)) - r^2 = 0;
        // => (t^2.d * d * 2td) * (C-Q)+(C-Q) * (C-Q) - r^2 = 0;
        //
        // on mapping this equation to quadratic discriminant formula
        // we get,
        // a = d * d;
        auto a = dot(r.direction(), r.direction());

        // -2d * (C - Q)
        auto b = /*-2.0 **/ dot(r.direction(), oc);

        // (C - Q) * (C - Q) - r^2
        auto c            = dot(oc, oc) - radius * radius;
        auto discriminant = b * b - /*4 **/ a * c;

        // get the discriminant if some
        if (discriminant < 0) {
            return false;
        }

        auto sqrtd = std::sqrt(discriminant);

        // Find closes root
        auto root = (b - sqrtd) / a;
        if (!ray_t.surrounds(root)) {
            root = (b + sqrtd) / a;
            if (!ray_t.surrounds(root)) {
                return false;
            }
        }

        rec.t           = root;
        rec.p           = r.at(rec.t);
        vec3 out_normal = (rec.p - center) / radius;
        rec.set_face_normal(r, out_normal);
        rec.mat = mat;

        return true;
    }

private:
    point3                    center;
    double                    radius;
    std::shared_ptr<material> mat;
};
