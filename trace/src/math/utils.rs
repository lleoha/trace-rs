use na::{Complex, UnitVector3};
use std::ops::Neg;

#[derive(Debug)]
pub struct Refraction {
    pub r_s: f32,
    pub r_p: f32,
}

pub fn fresnel(cos_theta_i: f32, eta: Complex<f32>) -> Refraction {
    let sin2_theta_i = 1.0 - cos_theta_i.powi(2);
    let sin2_theta_t = sin2_theta_i * eta * eta;
    let cos_theta_t = (1.0 - sin2_theta_t).sqrt();

    let r_s = (eta * cos_theta_i - cos_theta_t) / (eta * cos_theta_i + cos_theta_t);
    let r_p = (eta * cos_theta_t - cos_theta_i) / (eta * cos_theta_t + cos_theta_i);

    Refraction {
        r_s: r_s.norm_sqr(),
        r_p: r_p.norm_sqr(),
    }
}

pub fn refract(
    incoming: &UnitVector3<f32>,
    normal: &UnitVector3<f32>,
    eta: f32,
) -> UnitVector3<f32> {
    let cos_theta_i = -incoming.dot(normal.as_ref());
    let sin2_theta_i = 1.0 - cos_theta_i * cos_theta_i;
    let sin2_theta_t = sin2_theta_i * eta * eta;

    if sin2_theta_t >= 1.0 {
        // TIR
        reflect(incoming, normal)
    } else {
        let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
        UnitVector3::new_unchecked(
            incoming.as_ref() * eta + normal.as_ref() * (eta * cos_theta_i - cos_theta_t),
        )
    }
}

pub fn reflect(incoming: &UnitVector3<f32>, normal: &UnitVector3<f32>) -> UnitVector3<f32> {
    let n = normal.as_ref();
    let l = incoming.as_ref();

    UnitVector3::new_unchecked(n * n.dot(&l.neg()) * 2.0 + l)
}
