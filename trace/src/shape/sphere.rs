use crate::material::{DynMaterial, Material};
use crate::math::Ray;
use crate::shape::{Intersection, Shape};
use na::{Point3, UnitVector3};
use rand::Rng;

pub struct Sphere<R: Rng> {
    center: Point3<f32>,
    radius: f32,
    material: DynMaterial<R>,
}

impl<R: Rng> Sphere<R> {
    pub fn new(center: Point3<f32>, radius: f32, material: DynMaterial<R>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<R: Rng> Shape<R> for Sphere<R> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection<R>> {
        let half_b = ray.direction().dot(&(ray.origin() - self.center));
        let det =
            half_b.powi(2) - ((ray.origin() - self.center).norm_squared() - self.radius.powi(2));

        if det >= 0.0 {
            let det_sqrt = det.sqrt();
            let d1 = -half_b - det_sqrt;
            let d2 = -half_b + det_sqrt;
            let d = if d1 < 0.0 && d2 < 0.0 {
                None
            } else if d1 >= 0.0 {
                Some(d1)
            } else {
                Some(d2)
            };

            d.map(|distance| {
                let point = ray.origin() + (ray.direction().into_inner() * distance);
                Intersection {
                    distance,
                    point,
                    normal: UnitVector3::new_normalize(point - self.center),
                    shape: self,
                }
            })
        } else {
            None
        }
    }

    fn material(&self) -> &dyn Material<R> {
        self.material.as_ref()
    }
}
