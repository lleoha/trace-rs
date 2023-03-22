use na::{UnitVector3, Vector3};
use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::UnitSphere;

pub struct UnitHemisphere {
    direction: UnitVector3<f32>,
}

impl UnitHemisphere {
    pub fn new(direction: UnitVector3<f32>) -> Self {
        Self { direction }
    }
}

impl<T> From<T> for UnitHemisphere
where
    UnitVector3<f32>: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            direction: value.into(),
        }
    }
}

impl Distribution<UnitVector3<f32>> for UnitHemisphere {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnitVector3<f32> {
        loop {
            let sample = UnitSphere.sample(rng).into();
            if self.direction.dot(&sample) >= 0.0 {
                return UnitVector3::new_unchecked(sample);
            }
        }
    }
}

pub struct CosineUnitHemisphere {
    unit_sphere_distribution: UnitSphere,
    direction: UnitVector3<f32>,
}

impl CosineUnitHemisphere {
    pub fn new(direction: UnitVector3<f32>) -> Self {
        Self {
            unit_sphere_distribution: UnitSphere,
            direction,
        }
    }
}

impl<T> From<T> for CosineUnitHemisphere
where
    UnitVector3<f32>: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            unit_sphere_distribution: UnitSphere,
            direction: value.into(),
        }
    }
}

impl Distribution<UnitVector3<f32>> for CosineUnitHemisphere {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnitVector3<f32> {
        UnitVector3::new_normalize(
            self.direction.as_ref() * 1.005
                + Vector3::from(self.unit_sphere_distribution.sample(rng)),
        )
    }
}
