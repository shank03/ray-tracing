use crate::utils;

pub type Vec3 = [f64; 3];

pub type Point3 = Vec3;

pub fn empty() -> Vec3 {
    [0.0, 0.0, 0.0]
}

pub fn random() -> Vec3 {
    [
        utils::random_float(),
        utils::random_float(),
        utils::random_float(),
    ]
}

pub fn random_min_max(min: f64, max: f64) -> Vec3 {
    [
        utils::random_min_max(min, max),
        utils::random_min_max(min, max),
        utils::random_min_max(min, max),
    ]
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_min_max(-1.0, 1.0);
        let len_sq = p.len_sqr();

        if 1e-160 < len_sq && len_sq <= 1.0 {
            return p.div_f(len_sq.sqrt());
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = [
            utils::random_min_max(-1.0, 1.0),
            utils::random_min_max(-1.0, 1.0),
            0.0,
        ];
        if p.len_sqr() < 1.0 {
            return p;
        }
    }
}

pub trait VecOp {
    fn len_sqr(&self) -> f64;
    fn length(&self) -> f64;
    fn near_zero(&self) -> bool;
    fn dot(&self, v: &Vec3) -> f64;
    fn cross(&self, v: &Vec3) -> Vec3;
    fn unit(&self) -> Vec3;
    fn reflect(&self, n: &Vec3) -> Vec3;
    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3;

    fn neg(&self) -> Vec3;
    fn add_assign(&mut self, rhs: Vec3);

    fn add(&self, rhs: &Vec3) -> Vec3;
    fn sub(&self, rhs: &Vec3) -> Vec3;
    fn mul(&self, rhs: &Vec3) -> Vec3;
    fn mul_f(&self, rhs: f64) -> Vec3;
    fn div_f(&self, rhs: f64) -> Vec3;
}

impl VecOp for Vec3 {
    fn length(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    fn len_sqr(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    fn near_zero(&self) -> bool {
        self.iter().fold(true, |acc, i| acc && (i.abs() < 1e-8))
    }

    fn dot(&self, v: &Vec3) -> f64 {
        self.iter()
            .zip(v.iter())
            .fold(0.0, |acc, (u, v)| acc + (u * v))
    }

    fn cross(&self, v: &Vec3) -> Vec3 {
        [
            self[1] * v[2] - self[2] * v[1],
            self[2] * v[0] - self[0] * v[2],
            self[0] * v[1] - self[1] * v[0],
        ]
    }

    fn unit(&self) -> Vec3 {
        self.div_f(self.length())
    }

    fn reflect(&self, n: &Vec3) -> Vec3 {
        self.sub(&n.mul_f(2.0 * self.dot(n)))
    }

    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cosine = self.neg().dot(&n).min(1.0);
        let r_perp = self.add(&n.mul_f(cosine)).mul_f(etai_over_etat);
        let r_parallel = n.mul_f(-((1.0 - r_perp.len_sqr()).abs().sqrt()));
        r_perp.add(&r_parallel)
    }

    fn neg(&self) -> Vec3 {
        [-self[0], -self[1], -self[2]]
    }

    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }

    fn add(&self, rhs: &Vec3) -> Vec3 {
        [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
    }

    fn sub(&self, rhs: &Vec3) -> Vec3 {
        [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    }

    fn mul(&self, rhs: &Vec3) -> Vec3 {
        [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
    }

    fn mul_f(&self, rhs: f64) -> Vec3 {
        [self[0] * rhs, self[1] * rhs, self[2] * rhs]
    }

    fn div_f(&self, rhs: f64) -> Vec3 {
        self.mul_f(1.0 / rhs)
    }
}
