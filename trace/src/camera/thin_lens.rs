use crate::camera::Camera;
use crate::math::Ray;
use na::{Matrix4, Point3, UnitVector3, Vector2, Vector3};
use rand::Rng;
use rand_distr::{Distribution, UnitDisc};

pub struct ThinLensCamera {
    look_at_inv: Matrix4<f32>,
    aspect: f32,
    tan_half_fov_y: f32,
    aperture: f32,
    focal_distance: f32,
}

impl ThinLensCamera {
    pub fn new(
        eye: &Point3<f32>,
        target: &Point3<f32>,
        up: &Vector3<f32>,
        aspect: f32,
        fovy: f32,
        aperture: f32,
        focal_distance: f32,
    ) -> Self {
        let look_at = Matrix4::look_at_lh(eye, target, up);
        let look_at_inv = look_at.try_inverse().unwrap();
        let tan_half_fov_y = f32::tan(fovy / 2.0);

        Self {
            look_at_inv,
            aspect,
            tan_half_fov_y,
            aperture,
            focal_distance,
        }
    }
}

impl<R: Rng> Camera<R> for ThinLensCamera {
    fn sample(&self, rng: &mut R, x: f32, y: f32) -> Ray {
        let lens_radius = self.aperture * 0.5;
        let offset_vector =
            (Vector2::from(UnitDisc.sample(rng)) * lens_radius).fixed_resize::<3, 1>(0.0);

        let origin = Point3::from([0.0, 0.0, 0.0]) - offset_vector;
        let fy = self.tan_half_fov_y * y;
        let fx = self.tan_half_fov_y * x * self.aspect;
        let direction = UnitVector3::new_normalize([fx, fy, 1.0].into()).as_ref()
            * self.focal_distance
            + offset_vector;

        let world_origin = self.look_at_inv.transform_point(&origin);
        let world_direction = self.look_at_inv.transform_vector(&direction);
        Ray::from(world_origin, UnitVector3::new_normalize(world_direction))
    }
}
