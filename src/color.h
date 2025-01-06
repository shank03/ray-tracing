#pragma once

#include "interval.h"
#include "vec3.h"

using color = vec3;

inline void write_color(std::ostream &out, const color &pixel) {
    auto r = pixel.x();
    auto g = pixel.y();
    auto b = pixel.z();

    static const interval intensity(0.000, 0.999);

    int rb = int(256 * intensity.clamp(r));
    int gb = int(256 * intensity.clamp(g));
    int bb = int(256 * intensity.clamp(b));

    out << rb << ' ' << gb << ' ' << bb << '\n';
}
