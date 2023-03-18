pub mod pinhole;

use crate::math::ray::Ray;

pub trait Camera {
    fn sample(&self, x: f32, y: f32) -> Ray;
}
