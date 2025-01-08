use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::utils;

#[derive(Clone)]
#[repr(transparent)]
pub struct Vec3([f64; 3]);

pub type Point3 = Vec3;

impl Vec3 {
    pub fn empty() -> Self {
        Self([0.0, 0.0, 0.0])
    }

    pub fn new(v: [f64; 3]) -> Self {
        Self(v)
    }

    pub fn random() -> Self {
        Self([
            utils::random_float(),
            utils::random_float(),
            utils::random_float(),
        ])
    }

    pub fn random_min_max(min: f64, max: f64) -> Self {
        Self([
            utils::random_min_max(min, max),
            utils::random_min_max(min, max),
            utils::random_min_max(min, max),
        ])
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_min_max(-1.0, 1.0);
            let len_sq = p.len_sqr();

            if 1e-160 < len_sq && len_sq <= 1.0 {
                return &p / len_sq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let unit = Self::random_unit_vector();
        if unit.dot(&normal) > 0.0 {
            return unit;
        }
        -&unit
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self([
                utils::random_min_max(-1.0, 1.0),
                utils::random_min_max(-1.0, 1.0),
                0.0,
            ]);
            if p.len_sqr() < 1.0 {
                return p;
            }
        }
    }

    pub fn expand(self) -> (f64, f64, f64) {
        (self.0[0], self.0[1], self.0[2])
    }

    pub fn x(&self) -> &f64 {
        &self.0[0]
    }

    pub fn y(&self) -> &f64 {
        &self.0[1]
    }

    pub fn z(&self) -> &f64 {
        &self.0[2]
    }

    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn len_sqr(&self) -> f64 {
        self.0.iter().fold(0.0, |acc, i| acc + (i * i))
    }

    pub fn near_zero(&self) -> bool {
        self.0.iter().fold(true, |acc, i| acc && (i.abs() < 1e-8))
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.0
            .iter()
            .zip(v.0.iter())
            .fold(0.0, |acc, (u, v)| acc + (u * v))
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Self::new([
            self.0[1] * v.0[2] - self.0[2] * v.0[1],
            self.0[2] * v.0[0] - self.0[0] * v.0[2],
            self.0[0] * v.0[1] - self.0[1] * v.0[0],
        ])
    }

    pub fn unit(&self) -> Vec3 {
        self / self.len()
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - &(2.0 * self.dot(&n) * n)
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cosine = (-self).dot(&n).min(1.0);
        let r_perp = etai_over_etat * &(self + &(cosine * n));
        let r_parallel = -((1.0 - r_perp.len_sqr()).abs().sqrt()) * n;
        &r_perp + &r_parallel
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}
