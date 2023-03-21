use na::Vector3;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Spectrum {
    value: Vector3<f32>, // linear RGB values for now
}

impl Spectrum {
    pub fn new(value: Vector3<f32>) -> Self {
        Self { value }
    }

    pub fn zeros() -> Self {
        Self {
            value: Vector3::zeros(),
        }
    }

    pub fn to_srgb(&self) -> [f32; 3] {
        *self.value.map(Self::gamma).as_ref()
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
        let clamped = input.clamp(0.0, 1.0);
        if clamped <= 0.04045 {
            clamped / 12.92
        } else {
            ((clamped + 0.055) / 1.055).powf(2.4)
        }
    }
}

// from sRGB: u8
impl From<[u8; 3]> for Spectrum {
    fn from(value: [u8; 3]) -> Self {
        let value = value
            .map(|v| (v as f32) / 255.0)
            .map(Self::inverse_gamma)
            .into();

        Self { value }
    }
}

// from sRGB
impl From<[f32; 3]> for Spectrum {
    fn from(value: [f32; 3]) -> Self {
        let value = value.map(Self::inverse_gamma).into();

        Self { value }
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
