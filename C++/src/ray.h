#pragma once

#include "vec3.h"

class ray {
public:
    ray() = default;

    ray(const vec3& origin, const vec3& direction) : orig(origin), dir(direction) {}

    const vec3& origin() const { return orig; }

    const vec3& direction() const { return dir; }

    /// `P(t) = Q + td`
    ///
    /// `P` = 3D position
    /// `Q` = ray origin coordinates
    /// `d` = ray direction vector
    /// `t` = unit time
    ///
    /// returns position on the ray at time `t`
    vec3 at(double t) const {
        return orig + t * dir;
    }

private:
    vec3 orig;
    vec3 dir;
};
