#pragma once

#include <memory>
#include <vector>

#include "hittable.h"
#include "interval.h"

class hittable_list : public hittable {
public:
    std::vector<std::shared_ptr<hittable>> objects;

    hittable_list() = default;

    hittable_list(std::shared_ptr<hittable> obj) { add(obj); }

    void clear() { objects.clear(); }

    void add(std::shared_ptr<hittable> obj) {
        objects.push_back(obj);
    }

    bool hit(const ray &r, interval ray_t, hit_record &rec) const override {
        hit_record temp_rec;
        bool       hit     = false;
        auto       closest = ray_t.max;

        for (const auto &obj : objects) {
            if (obj->hit(r, interval(ray_t.min, closest), temp_rec)) {
                hit     = true;
                closest = temp_rec.t;
                rec     = temp_rec;
            }
        }

        return hit;
    }
};
