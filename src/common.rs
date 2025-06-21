pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use rand;
pub use rand::Rng;

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // it returns [min, max)
    min + (max - min) * random_double()
}
