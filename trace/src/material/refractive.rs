use na::{Complex, ComplexField};

pub struct Refraction {
    r_s: f32,
    r_p: f32,
}

pub fn fresnel(cos_theta_i: f32, eta_i: Complex<f32>, eta_t: Complex<f32>) -> Refraction {
    let eta = eta_i / eta_t;
    let sin_theta_i = (1.0 - cos_theta_i.powi(2)).sqrt();
    let sin_theta_t = sin_theta_i * eta;
    let cos_theta_t = (1.0 - sin_theta_t.powi(2)).sqrt();

    let r_s = (eta * cos_theta_i - cos_theta_t) / (eta * cos_theta_i + cos_theta_t);
    let r_p = (eta * cos_theta_t - cos_theta_i) / (eta * cos_theta_t + cos_theta_i);

    Refraction {
        r_s: r_s.norm_sqr(),
        r_p: r_p.norm_sqr(),
    }
}

// TODO
