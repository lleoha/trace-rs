mod pinhole;

pub use self::pinhole::PinholeCamera;
use crate::math::Ray;

pub trait Camera {
    fn sample(&self, x: f32, y: f32) -> Ray;
}

pub type DynCamera = Box<dyn Camera + Send + Sync>;
