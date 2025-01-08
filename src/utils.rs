use std::f64::consts::PI;

use rand::Rng;

pub fn degress_to_radians(d: f64) -> f64 {
    d * PI / 180.0
}

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_min_max(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}
