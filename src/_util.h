#pragma once

#include <cmath>
#include <random>
#include <iostream>
#include <limits>
#include <memory>

// Const
const double infinity = std::numeric_limits<double>::infinity();
const double pi       = 3.1415926535897932385;

// Func
inline double degress_to_radians(double d) {
    return d * pi / 180.0;
}

inline double random_double() {
    static std::uniform_real_distribution<double> dist(0.0, 1.0);
    static std::mt19937 gen;
    return dist(gen);
}

inline double random_double(double min, double max) {
    return min + (max - min) * random_double();
}

// Headers
#include "color.h"
#include "interval.h"
#include "ray.h"
#include "vec3.h"
