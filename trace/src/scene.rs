use crate::camera::{Camera, DynCamera};
use crate::math::Ray;
use crate::shape::{DynShape, Intersection};
use rand::Rng;

pub struct Scene<R: Rng> {
    camera: DynCamera,
    objects: Vec<DynShape<R>>,
}

impl<R: Rng> Scene<R> {
    pub fn new(camera: DynCamera, objects: Vec<DynShape<R>>) -> Self {
        Self { camera, objects }
    }

    pub fn add(&mut self, object: DynShape<R>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection<R>> {
        let mut nearest_intersection: Option<Intersection<R>> = None;

        for obj in self.objects.as_slice() {
            let intersection = obj.intersect(ray);
            let nearest_distance = nearest_intersection
                .as_ref()
                .map_or(f32::MAX, |i| i.distance);
            let distance = intersection.as_ref().map_or(f32::MAX, |i| i.distance);
            if distance < nearest_distance {
                nearest_intersection = intersection
            }
        }

        nearest_intersection
    }

    pub fn camera(&self) -> &dyn Camera {
        self.camera.as_ref()
    }
}
