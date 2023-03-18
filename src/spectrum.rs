pub mod cie;

use na::Vector3;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Spectrum {
    value: Vector3<f32>, // RGB values for now
}

impl Spectrum {
    pub fn new<T: Into<Vector3<f32>>>(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn from_rgb(value: [u8; 3]) -> Self {
        let value = value
            .map(|v| (v as f32) / 255.0)
            .map(Self::inverse_gamma)
            .into();

        Self { value }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        *self
            .value
            .map(Self::gamma)
            .map(|v| (v * 255.0) as u8)
            .as_ref()
    }

    pub fn to_inner(&self) -> Vector3<f32> {
        self.value
    }

    fn gamma(input: f32) -> f32 {
        if input <= 0.0031308 {
            12.92 * input
        } else {
            1.055 * input.powf(1.0 / 2.4) - 0.055
        }
        .clamp(0.0, 1.0)
    }

    fn inverse_gamma(input: f32) -> f32 {
        if input <= 0.04045 {
            input / 12.92
        } else {
            ((input + 0.055) / 1.055).powf(2.4)
        }
    }
}

impl Add<Spectrum> for Spectrum {
    type Output = Spectrum;

    fn add(self, rhs: Spectrum) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl Mul<Spectrum> for Spectrum {
    type Output = Spectrum;

    fn mul(self, rhs: Spectrum) -> Self::Output {
        Self::Output {
            value: self.value.component_mul(&rhs.value),
        }
    }
}

impl Mul<f32> for Spectrum {
    type Output = Spectrum;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            value: self.value * rhs,
        }
    }
}

impl<T> From<T> for Spectrum
where
    Vector3<f32>: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}
