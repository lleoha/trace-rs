use crate::camera::Camera;
use crate::math::ray::Ray;
use na::{Matrix4, Point3, UnitVector3, Vector3};

pub struct PinholeCamera {
    look_at_inv: Matrix4<f32>,
    aspect: f32,
    tan_half_fov_y: f32,
}

impl PinholeCamera {
    pub fn new(
        eye: &Point3<f32>,
        target: &Point3<f32>,
        up: &Vector3<f32>,
        aspect: f32,
        fovy: f32,
    ) -> Self {
        let look_at = Matrix4::look_at_lh(eye, target, up);
        let look_at_inv = look_at.try_inverse().unwrap();
        let tan_half_fov_y = f32::tan(fovy / 2.0);

        Self {
            look_at_inv,
            aspect,
            tan_half_fov_y,
        }
    }
}

impl Camera for PinholeCamera {
    fn sample(&self, x: f32, y: f32) -> Ray {
        let origin = self
            .look_at_inv
            .transform_point(&Point3::from([0.0, 0.0, 0.0]));
        let fy = self.tan_half_fov_y * y;
        let fx = self.tan_half_fov_y * x * self.aspect;
        let direction = self
            .look_at_inv
            .transform_vector(&Vector3::from([fx, fy, 1.0]));

        Ray::new(origin, UnitVector3::new_normalize(direction))
    }
}
