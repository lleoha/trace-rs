use na::{Complex, UnitVector3};
use rand::Rng;
use crate::material::Material;
use crate::math::utils::{fresnel, reflect};
use crate::shape::Intersection;
use crate::spectrum::Spectrum;

pub struct Metal {
    refraction_index: Complex<f32>,
}

impl Metal {
    pub fn new(refraction_index: Complex<f32>) -> Self {
        Self {
            refraction_index,
        }
    }
}

impl<R: Rng> Material<R> for Metal {
    fn scatter(&self, intersection: &Intersection<R>, incoming: &UnitVector3<f32>, rng: &mut R) -> UnitVector3<f32> {
        reflect(incoming, &intersection.normal)
    }

    fn attenuation(&self, intersection: &Intersection<R>, incoming: &UnitVector3<f32>, outgoing: &UnitVector3<f32>) -> Spectrum {
        let cos_theta_i = -incoming.dot(intersection.normal.as_ref());
        let angle = cos_theta_i.acos().to_degrees();
        let eta = 1.0 / self.refraction_index;
        let f = fresnel(cos_theta_i, eta);
        let reflective_attenuation = (f.r_p + f.r_s) * 0.5; // un-polarized light

        Spectrum::ones() * reflective_attenuation
    }

    fn emission(&self, intersection: &Intersection<R>, incoming: &UnitVector3<f32>, outgoing: &UnitVector3<f32>) -> Spectrum {
        Spectrum::zeros()
    }
}
