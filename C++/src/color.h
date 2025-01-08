#pragma once

#include "interval.h"
#include "vec3.h"

using color = vec3;

inline double linear_to_gamma(double linear_comp) {
    if (linear_comp > 0) {
        return std::sqrt(linear_comp);
    }
    return 0;
}

inline void write_color(std::ostream &out, const color &pixel) {
    auto r = pixel.x();
    auto g = pixel.y();
    auto b = pixel.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    static const interval intensity(0.000, 0.999);

    int rb = int(256 * intensity.clamp(r));
    int gb = int(256 * intensity.clamp(g));
    int bb = int(256 * intensity.clamp(b));

    out << rb << ' ' << gb << ' ' << bb << '\n';
}
