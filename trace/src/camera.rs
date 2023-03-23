mod pinhole;
mod thin_lens;

pub use self::pinhole::PinholeCamera;
pub use self::thin_lens::ThinLensCamera;
use crate::math::Ray;
use rand::Rng;

pub trait Camera<R: Rng> {
    fn sample(&self, rng: &mut R, x: f32, y: f32) -> Ray;
}

pub type DynCamera<R> = dyn Camera<R> + Send + Sync;
