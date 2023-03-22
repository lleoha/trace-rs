mod lambertian;
mod refractive;
mod specular;

pub use self::lambertian::Lambertian;
pub use self::specular::Specular;
use crate::shape::Intersection;
use crate::spectrum::Spectrum;
use na::UnitVector3;
use rand::Rng;

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
