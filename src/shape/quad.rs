use crate::material::Material;
use crate::math::ray::Ray;
use crate::shape::{Intersection, Shape};
use na::{Point3, UnitVector3, Vector3};
use rand::Rng;
use rand_distr::num_traits::{Signed, Zero};

use std::ops::Neg;

pub struct Quad<R: Rng> {
    p0: Point3<f32>,
    v1: Vector3<f32>,
    v2: Vector3<f32>,
    material: Box<dyn Material<R>>,
}

impl<R: Rng> Quad<R> {
    pub fn new(
        p0: Point3<f32>,
        v1: Vector3<f32>,
        v2: Vector3<f32>,
        material: Box<dyn Material<R>>,
    ) -> Self {
        Self {
            p0,
            v1,
            v2,
            material,
        }
    }

    pub fn from_points<P>(p0: P, p1: P, p2: P, material: Box<dyn Material<R>>) -> Self
    where
        P: Into<Point3<f32>>,
    {
        let p0_point = p0.into();
        let p1_point = p1.into();
        let v1 = p1_point - p0_point;
        let v2 = p2.into() - p1_point;
        Self {
            p0: p0_point,
            v1,
            v2,
            material,
        }
    }
}

impl<R: Rng> Shape<R> for Quad<R> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection<R>> {
        let v1_cross_v2 = self.v1.cross(&self.v2);

        let det = ray.direction().neg().dot(&v1_cross_v2);
        if det.is_zero() {
            None
        } else {
            let t = v1_cross_v2.dot(&(ray.origin() - self.p0)) / det;
            let u = self
                .v2
                .cross(ray.direction().neg().as_ref())
                .dot(&(ray.origin() - self.p0))
                / det;
            let v = ray
                .direction()
                .neg()
                .cross(&self.v1)
                .dot(&(ray.origin() - self.p0))
                / det;

            if t.is_positive() && (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) {
                Some(Intersection {
                    distance: t,
                    point: ray.origin() + ray.direction().into_inner() * t,
                    normal: UnitVector3::new_normalize(v1_cross_v2),
                    shape: self,
                })
            } else {
                None
            }
        }
    }

    fn material(&self) -> &dyn Material<R> {
        self.material.as_ref()
    }
}
