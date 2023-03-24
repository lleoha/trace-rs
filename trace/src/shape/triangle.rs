use crate::material::{DynMaterial, Material};
use crate::math::Ray;
use crate::shape::{Intersection, Shape};
use na::{Point3, UnitVector3, Vector3};
use rand::Rng;

pub struct Triangle<R: Rng> {
    p0: Point3<f32>,
    v1: Vector3<f32>,
    v2: Vector3<f32>,
    material: Box<DynMaterial<R>>,
}

impl<R: Rng> Triangle<R> {
    const EPSILON: f32 = 0.0000001;

    pub fn from_points<M: Material<R> + Send + Sync + 'static>(
        p0: &Point3<f32>,
        p1: &Point3<f32>,
        p2: &Point3<f32>,
        material: M,
    ) -> Self {
        Self {
            p0: *p0,
            v1: p1 - p0,
            v2: p2 - p0,
            material: Box::new(material),
        }
    }
}

impl<R: Rng> Shape<R> for Triangle<R> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection<R>> {
        // Möller–Trumbore intersection algorithm
        let h = ray.direction().cross(&self.v2);
        let a = self.v1.dot(&h);
        if (-Self::EPSILON..=Self::EPSILON).contains(&a) {
            return None;
        }
        let f = 1.0 / a;
        let s = ray.origin() - self.p0;
        let u = f * s.dot(&h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(&self.v1);
        let v = f * ray.direction().dot(&q);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let distance = f * self.v2.dot(&q);
        if distance < Self::EPSILON {
            return None;
        }

        let point = ray.origin() + ray.direction().as_ref() * distance;
        Some(Intersection {
            distance,
            point,
            normal: UnitVector3::new_normalize(self.v1.cross(&self.v2)),
            shape: self,
        })
    }

    fn material(&self) -> &dyn Material<R> {
        self.material.as_ref()
    }
}
