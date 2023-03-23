use crate::camera::{Camera, DynCamera};
use crate::math::Ray;
use crate::shape::{DynShape, Intersection, Shape};
use rand::Rng;

pub struct Scene<R: Rng> {
    camera: Box<DynCamera<R>>,
    objects: Vec<Box<DynShape<R>>>,
}

impl<R: Rng> Scene<R> {
    pub fn new<C: Camera<R> + Send + Sync + 'static>(camera: C) -> Self {
        Self {
            camera: Box::new(camera),
            objects: Vec::new(),
        }
    }

    pub fn add<S: Shape<R> + Send + Sync + 'static>(&mut self, shape: S) -> &mut Self {
        self.objects.push(Box::new(shape));
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

    pub fn camera(&self) -> &dyn Camera<R> {
        self.camera.as_ref()
    }
}
