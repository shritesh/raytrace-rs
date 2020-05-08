use rand::random;
use std::f64::consts::PI;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    random::<f64>()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}
