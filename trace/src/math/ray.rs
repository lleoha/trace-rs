use na::{Point3, UnitVector3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point3<f32>,
    direction: UnitVector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: UnitVector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn from<P, V>(origin: P, direction: V) -> Self
    where
        Point3<f32>: From<P>,
        UnitVector3<f32>: From<V>,
    {
        Self {
            origin: origin.into(),
            direction: direction.into(),
        }
    }

    pub fn origin(&self) -> &Point3<f32> {
        &self.origin
    }

    pub fn direction(&self) -> &UnitVector3<f32> {
        &self.direction
    }
}
