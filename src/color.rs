use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    0.0
}

pub fn get_pixel(color: Color) -> [u8; 3] {
    let [r, g, b] = color;
    let [r, g, b] = [linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b)];

    let range = 0.000..0.999;

    let rb = (256.0 * range.clamp(r)) as u8;
    let gb = (256.0 * range.clamp(g)) as u8;
    let bb = (256.0 * range.clamp(b)) as u8;

    [rb, gb, bb]
}
