use crate::color::data::{CIE_XYZ_1931_2DEG_10NM, RGB_COMPONENTS, XYZ_TO_LINEAR_RGB};
use nalgebra::{SVector, Vector3};
use std::ops::{Add, Mul};

// represents spectrum 380-780 nm in 10 nm bins (hence 41 bins)
#[derive(Copy, Clone)]
pub struct Spectrum {
    inner: SVector<f32, 41>,
}

impl Spectrum {
    pub fn new(inner: [f32; 41]) -> Self {
        Self {
            inner: inner.into(),
        }
    }

    pub fn from_linear_rgb(linear_rgb: &Vector3<f32>) -> Self {
        let r = RGB_COMPONENTS.column(0) * linear_rgb.x;
        let g = RGB_COMPONENTS.column(1) * linear_rgb.y;
        let b = RGB_COMPONENTS.column(2) * linear_rgb.z;

        Self { inner: r + g + b }
    }

    pub fn from_srgb(srgb: &Vector3<f32>) -> Self {
        Self::from_linear_rgb(&srgb.map(Self::inverse_gamma))
    }

    pub fn from_srgb_u8(srgb: &Vector3<u8>) -> Self {
        Self::from_srgb(&srgb.map(|v| (v as f32) / 255.0))
    }

    pub fn black() -> Self {
        Self {
            inner: SVector::zeros(),
        }
    }

    pub fn white() -> Self {
        Self {
            inner: SVector::from_element(1.0),
        }
    }

    pub fn to_inner(&self) -> SVector<f32, 41> {
        self.inner.to_owned()
    }

    pub fn to_xyz(&self) -> Vector3<f32> {
        let scale: f32 = CIE_XYZ_1931_2DEG_10NM.column(1).sum();
        CIE_XYZ_1931_2DEG_10NM.transpose() * self.inner / scale
    }

    pub fn to_linear_rgb(&self) -> Vector3<f32> {
        XYZ_TO_LINEAR_RGB * self.to_xyz()
    }

    pub fn to_srgb(&self) -> Vector3<f32> {
        self.to_linear_rgb().map(Self::gamma)
    }

    fn gamma(x: f32) -> f32 {
        if x >= 0.0 {
            if x <= 0.0031308 {
                12.92 * x
            } else {
                1.055 * x.powf(1.0 / 2.4) - 0.055
            }
        } else {
            -Self::gamma(-x)
        }
    }

    fn inverse_gamma(x: f32) -> f32 {
        if x >= 0.0 {
            if x <= 0.04045 {
                x / 12.92
            } else {
                ((x + 0.055) / 1.055).powf(2.4)
            }
        } else {
            -Self::inverse_gamma(-x)
        }
    }
}

impl From<SVector<f32, 41>> for Spectrum {
    fn from(value: SVector<f32, 41>) -> Self {
        Self { inner: value }
    }
}

impl From<&SVector<f32, 41>> for Spectrum {
    fn from(value: &SVector<f32, 41>) -> Self {
        Self {
            inner: *value,
        }
    }
}

impl Add<Spectrum> for Spectrum {
    type Output = Self;

    fn add(self, rhs: Spectrum) -> Self::Output {
        Self::Output {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Add<&Spectrum> for Spectrum {
    type Output = Self;

    fn add(self, rhs: &Spectrum) -> Self::Output {
        Self::Output {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Mul<Spectrum> for Spectrum {
    type Output = Self;

    fn mul(self, rhs: Spectrum) -> Self::Output {
        Self::Output {
            inner: self.inner.component_mul(&rhs.inner),
        }
    }
}

impl Mul<&Spectrum> for Spectrum {
    type Output = Self;

    fn mul(self, rhs: &Spectrum) -> Self::Output {
        Self::Output {
            inner: self.inner.component_mul(&rhs.inner),
        }
    }
}

impl Mul<f32> for Spectrum {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            inner: self.inner * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::data::D65_ILLUM;
    
    use std::ops::Deref;

    #[test]
    fn test() {
        let spec = Spectrum::from_srgb(&[1.0, 1.0, 1.0].into())
            * Spectrum::from(D65_ILLUM.deref())
            * (1.0 / 100.0);
        let rgb = spec.to_srgb();
        println!("{}", &rgb);
    }
}
