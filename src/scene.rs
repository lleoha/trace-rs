use crate::math::ray::Ray;
use crate::shape::{Intersection, Shape};
use rand::Rng;

pub struct Scene<R: Rng + ?Sized> {
    objects: Vec<Box<dyn Shape<R>>>,
}

impl<R: Rng> Scene<R> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Shape<R> + 'static) -> &mut Self {
        self.objects.push(Box::new(object));
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
}

impl<R: Rng> Default for Scene<R> {
    fn default() -> Self {
        Self::new()
    }
}
