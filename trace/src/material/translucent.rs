use crate::color::spectrum::Spectrum;
use crate::material::Material;
use crate::math::utils::{fresnel, reflect, refract};
use crate::shape::Intersection;
use na::UnitVector3;
use rand::Rng;

pub struct Translucent {
    refractive_index: f32,
}

impl Translucent {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }
}

impl<R: Rng> Material<R> for Translucent {
    fn scatter(
        &self,
        intersection: &Intersection<R>,
        incoming: &UnitVector3<f32>,
        rng: &mut R,
    ) -> UnitVector3<f32> {
        let c = incoming.dot(intersection.normal.as_ref());
        let (cos_theta_i, n, eta) = if c <= 0.0 {
            // going inside
            (-c, intersection.normal, 1.0 / self.refractive_index)
        } else {
            // going outside
            (c, -intersection.normal, self.refractive_index)
        };

        let fresnel_terms = fresnel(cos_theta_i, eta.into());
        let r = (fresnel_terms.r_s + fresnel_terms.r_p) * 0.5;

        if rng.gen::<f32>() < r {
            reflect(incoming, &n)
        } else {
            refract(incoming, &n, eta)
        }
    }

    fn attenuation(
        &self,
        _intersection: &Intersection<R>,
        _incoming: &UnitVector3<f32>,
        _outgoing: &UnitVector3<f32>,
    ) -> Spectrum {
        Spectrum::white()
    }

    fn emission(
        &self,
        _intersection: &Intersection<R>,
        _incoming: &UnitVector3<f32>,
        _outgoing: &UnitVector3<f32>,
    ) -> Spectrum {
        Spectrum::black()
    }
}
