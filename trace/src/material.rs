mod lambertian;
mod metal;
mod refractive;
mod specular;
mod translucent;

pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
pub use self::specular::Specular;
pub use self::translucent::Translucent;
use crate::shape::Intersection;
use na::UnitVector3;
use rand::Rng;
use crate::color::spectrum::Spectrum;

pub trait Material<R: Rng> {
    fn scatter(
        &self,
        intersection: &Intersection<R>,
        incoming: &UnitVector3<f32>,
        rng: &mut R,
    ) -> UnitVector3<f32>;

    fn attenuation(
        &self,
        intersection: &Intersection<R>,
        incoming: &UnitVector3<f32>,
        outgoing: &UnitVector3<f32>,
    ) -> Spectrum;

    fn emission(
        &self,
        intersection: &Intersection<R>,
        incoming: &UnitVector3<f32>,
        outgoing: &UnitVector3<f32>,
    ) -> Spectrum;
}

pub type DynMaterial<R> = dyn Material<R> + Send + Sync;
