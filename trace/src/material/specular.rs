use crate::color::spectrum::Spectrum;
use crate::material::Material;
use crate::shape::Intersection;
use na::UnitVector3;
use rand::Rng;

use crate::math::utils::reflect;

#[derive(Default)]
pub struct Specular {}

impl<R: Rng> Material<R> for Specular {
    fn scatter(
        &self,
        intersection: &Intersection<R>,
        incoming: &UnitVector3<f32>,
        _: &mut R,
    ) -> UnitVector3<f32> {
        reflect(incoming, &intersection.normal)
    }

    fn attenuation(
        &self,
        _: &Intersection<R>,
        _: &UnitVector3<f32>,
        _: &UnitVector3<f32>,
    ) -> Spectrum {
        Spectrum::from_linear_rgb(&[0.99, 0.99, 0.99].into())
    }

    fn emission(
        &self,
        _: &Intersection<R>,
        _: &UnitVector3<f32>,
        _: &UnitVector3<f32>,
    ) -> Spectrum {
        Spectrum::black()
    }
}
