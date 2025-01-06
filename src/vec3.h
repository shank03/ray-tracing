#pragma once

#include "_util.h"

class vec3 {
public:
    double v[3];

    vec3() : v { 0, 0, 0 } {}

    vec3(double v0, double v1, double v2) : v { v0, v1, v2 } {}

    double x() const { return v[0]; }

    double y() const { return v[1]; }

    double z() const { return v[2]; }

    vec3 operator-() const { return vec3(-v[0], -v[1], -v[2]); }

    double operator[](int i) const { return v[i]; }

    double& operator[](int i) { return v[i]; }

    vec3& operator+=(const vec3& nv) {
        v[0] += nv.v[0];
        v[1] += nv.v[1];
        v[2] += nv.v[2];
        return *this;
    }

    vec3& operator*=(double t) {
        v[0] *= t;
        v[1] *= t;
        v[2] *= t;
        return *this;
    }

    vec3& operator/=(double t) {
        return *this *= 1 / t;
    }

    double length() const {
        return std::sqrt(length_squared());
    }

    double length_squared() const {
        return v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    }

    bool near_zero() const {
        auto s = 1e-8;
        return (std::fabs(v[0]) < s) && (std::fabs(v[1]) < s) && (std::fabs(v[2]) < s);
    }

    static vec3 random() {
        return vec3(random_double(), random_double(), random_double());
    }

    static vec3 random(double min, double max) {
        return vec3(random_double(min, max), random_double(min, max), random_double(min, max));
    }
};

using point3 = vec3;

inline std::ostream& operator<<(std::ostream& out, const vec3& v) {
    return out << v.v[0] << ' ' << v.v[1] << ' ' << v.v[2];
}

inline vec3 operator+(const vec3& u, const vec3& v) {
    return vec3(u.v[0] + v.v[0], u.v[1] + v.v[1], u.v[2] + v.v[2]);
}

inline vec3 operator-(const vec3& u, const vec3& v) {
    return vec3(u.v[0] - v.v[0], u.v[1] - v.v[1], u.v[2] - v.v[2]);
}

inline vec3 operator*(const vec3& u, const vec3& v) {
    return vec3(u.v[0] * v.v[0], u.v[1] * v.v[1], u.v[2] * v.v[2]);
}

inline vec3 operator*(double t, const vec3& v) {
    return vec3(t * v.v[0], t * v.v[1], t * v.v[2]);
}

inline vec3 operator*(const vec3& v, double t) {
    return t * v;
}

inline vec3 operator/(const vec3& v, double t) {
    return (1 / t) * v;
}

inline double dot(const vec3& u, const vec3& v) {
    return u.v[0] * v.v[0]
           + u.v[1] * v.v[1]
           + u.v[2] * v.v[2];
}

inline vec3 cross(const vec3& u, const vec3& v) {
    return vec3(u.v[1] * v.v[2] - u.v[2] * v.v[1],
                u.v[2] * v.v[0] - u.v[0] * v.v[2],
                u.v[0] * v.v[1] - u.v[1] * v.v[0]);
}

inline vec3 unit_vector(const vec3& v) {
    return v / v.length();
}

inline vec3 random_unit_vector() {
    while (true) {
        auto p      = vec3::random(-1, 1);
        auto lens_q = p.length_squared();

        if (1e-160 < lens_q && lens_q <= 1) return p / std::sqrt(lens_q);
    }
}

inline vec3 random_on_hemisphere(const vec3& normal) {
    vec3 on_unit_sphere = random_unit_vector();
    if (dot(on_unit_sphere, normal) > 0.0) {
        return on_unit_sphere;
    }
    return -on_unit_sphere;
}

inline vec3 reflect(const vec3& v, const vec3& n) {
    return v - 2 * dot(v, n) * n;
}

inline vec3 refract(const vec3& uv, const vec3& n, double etai_over_etat) {
    auto cos        = std::fmin(dot(-uv, n), 1.0);
    vec3 r_perp     = etai_over_etat * (uv + cos * n);
    vec3 r_parallel = -std::sqrt(std::fabs(1.0 - r_perp.length_squared())) * n;
    return r_perp + r_parallel;
}
