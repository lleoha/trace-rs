pub mod quad;
pub mod sphere;

use crate::material::Material;
use crate::math::ray::Ray;
use na::{Point3, UnitVector3};
use rand::Rng;

pub struct Intersection<'a, R: Rng> {
    pub distance: f32,
    pub point: Point3<f32>,
    pub normal: UnitVector3<f32>,
    pub shape: &'a dyn Shape<R>,
}

pub trait Shape<R: Rng> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection<R>>;
    fn material(&self) -> &dyn Material<R>;
}
