#pragma once

#include "interval.h"
#include "ray.h"
#include "vec3.h"

typedef struct hr {
    point3 p;
    vec3   normal;
    double t;
    bool   front_face;

    void set_face_normal(const ray &r, const vec3 &out_normal) {
        // Sets hit record for normal vector
        // `out_normal` vec is assumed to have unit length

        // if front face > 0 then ray is inside the sphere else outside the sphere
        front_face = dot(r.direction(), out_normal) < 0;
        normal     = front_face ? out_normal : -out_normal;
    }
} hit_record;

class hittable {
public:
    virtual ~hittable() = default;

    virtual bool hit(const ray &r, interval ray_t, hit_record &rec) const = 0;
};
