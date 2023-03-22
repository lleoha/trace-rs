use crate::material::Material;
use crate::shape::Intersection;
use crate::spectrum::Spectrum;
use na::{UnitVector3, Vector3};
use rand::Rng;
use std::ops::Neg;

#[derive(Default)]
pub struct Specular {}

impl Specular {
    fn reflect(incoming: &UnitVector3<f32>, normal: &UnitVector3<f32>) -> UnitVector3<f32> {
        let n = normal.as_ref();
        let l = incoming.as_ref();

        UnitVector3::new_unchecked(n * n.dot(&l.neg()) * 2.0 + l)
    }
}

impl<R: Rng> Material<R> for Specular {
    fn scatter(
        &self,
        intersection: &Intersection<R>,
        incoming: &UnitVector3<f32>,
        _: &mut R,
    ) -> UnitVector3<f32> {
        Self::reflect(incoming, &intersection.normal)
    }

    fn attenuation(
        &self,
        _: &Intersection<R>,
        _: &UnitVector3<f32>,
        _: &UnitVector3<f32>,
    ) -> Spectrum {
        Spectrum::new(Vector3::new(0.9, 0.9, 0.9))
    }

    fn emission(
        &self,
        _: &Intersection<R>,
        _: &UnitVector3<f32>,
        _: &UnitVector3<f32>,
    ) -> Spectrum {
        Spectrum::new(Vector3::zeros())
    }
}
