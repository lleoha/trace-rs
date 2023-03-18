use crate::material::Material;
use crate::math::distribution::CosineUnitHemisphere;
use crate::shape::Intersection;
use crate::spectrum::Spectrum;
use na::{UnitVector3, Vector3};
use rand::Rng;
use rand_distr::Distribution;

pub struct Lambertian {
    albedo: Spectrum,
    emission: Spectrum,
}

impl Lambertian {
    pub fn new(albedo: Spectrum, emission: Spectrum) -> Self {
        Self { albedo, emission }
    }
}

impl<R: Rng> Material<R> for Lambertian {
    fn scatter(
        &self,
        intersection: &Intersection<R>,
        _: &UnitVector3<f32>,
        rng: &mut R,
    ) -> UnitVector3<f32> {
        let cosine_unit_sphere = CosineUnitHemisphere::new(intersection.normal);

        cosine_unit_sphere.sample(rng)
    }

    fn attenuation(
        &self,
        _: &Intersection<R>,
        _: &UnitVector3<f32>,
        _: &UnitVector3<f32>,
    ) -> Spectrum {
        self.albedo
    }

    fn emission(
        &self,
        intersection: &Intersection<R>,
        incoming: &UnitVector3<f32>,
        _: &UnitVector3<f32>,
    ) -> Spectrum {
        if incoming.dot(&intersection.normal) < 0.0 {
            self.emission
        } else {
            Spectrum::from(Vector3::zeros())
        }
    }
}
