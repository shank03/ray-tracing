#pragma once

#include <iostream>

#include "vec3.h"

using color = vec3;

void write_color(std::ostream &out, const color &pixel) {
    auto r = pixel.x();
    auto g = pixel.y();
    auto b = pixel.z();

    int rb = int(255.999 * r);
    int gb = int(255.999 * g);
    int bb = int(255.999 * b);

    out << rb << ' ' << gb << ' ' << bb << '\n';
}
