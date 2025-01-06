#pragma once

#include <cmath>
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

// Headers
#include "color.h"
#include "interval.h"
#include "ray.h"
#include "vec3.h"
