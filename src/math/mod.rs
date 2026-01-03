mod interval;
mod ray;
mod vec3;

pub use interval::*;
pub use ray::*;
pub use vec3::*;

pub fn random_double(min: f32, max: f32) -> f32 {
    let random_fraction: f32 = rand::random();
    min + (max - min) * random_fraction
}
