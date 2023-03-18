use crate::camera::pinhole::PinholeCamera;
use crate::material::lambertian::Lambertian;
use crate::material::specular::Specular;
use crate::scene::Scene;
use crate::shape::quad::Quad;
use crate::shape::sphere::Sphere;
use crate::spectrum::Spectrum;
use rand::Rng;

pub fn create_cornell_box<R: Rng + 'static>() -> (PinholeCamera, Scene<R>) {
    let floor_material = Lambertian::new(
        Spectrum::from_rgb([255, 255, 255]),
        Spectrum::from_rgb([0, 0, 0]),
    );
    let floor = Quad::from_points(
        [550.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 550.0],
        Box::new(floor_material),
    );

    let light_material = Lambertian::new(
        Spectrum::new([0.78, 0.78, 0.78]),
        Spectrum::new([4.5, 4.0, 2.0]),
    );
    let light = Quad::from_points(
        [343.0, 549.9, 227.0],
        [343.0, 549.9, 332.0],
        [213.0, 549.9, 332.0],
        Box::new(light_material),
    );

    let ceiling_material = Lambertian::new(
        Spectrum::from_rgb([255, 255, 255]),
        Spectrum::from_rgb([0, 0, 0]),
    );
    let ceiling = Quad::from_points(
        [550.0, 550.0, 0.0],
        [550.0, 550.0, 550.0],
        [0.0, 550.0, 550.0],
        Box::new(ceiling_material),
    );

    let back_wall_material = Lambertian::new(
        Spectrum::from_rgb([255, 255, 255]),
        Spectrum::from_rgb([0, 0, 0]),
    );
    let back_wall = Quad::from_points(
        [550.0, 0.0, 550.0],
        [0.0, 0.0, 550.0],
        [0.0, 550.0, 550.0],
        Box::new(back_wall_material),
    );

    let right_wall_material = Lambertian::new(
        Spectrum::from_rgb([255, 0, 0]),
        Spectrum::from_rgb([0, 0, 0]),
    );
    let right_wall = Quad::from_points(
        [0.0, 0.0, 550.0],
        [0.0, 0.0, 0.0],
        [0.0, 550.0, 0.0],
        Box::new(right_wall_material),
    );

    let left_wall_material = Lambertian::new(
        Spectrum::from_rgb([0, 255, 0]),
        Spectrum::from_rgb([0, 0, 0]),
    );
    let left_wall = Quad::from_points(
        [550.0, 0.0, 0.0],
        [550.0, 0.0, 550.0],
        [550.0, 550.0, 550.0],
        Box::new(left_wall_material),
    );

    let sphere1_material = Specular::default();
    let sphere_1 = Sphere::new([294.0, 120.0, 350.0], 120.0, sphere1_material);

    let sphere2_material = Specular::default();
    let sphere_2 = Sphere::new([400.0, 80.0, 150.0], 80.0, sphere2_material);

    let mut scene = Scene::new();
    scene
        .add(floor)
        .add(ceiling)
        .add(back_wall)
        .add(left_wall)
        .add(right_wall)
        .add(sphere_1)
        .add(sphere_2)
        .add(light);

    let camera = PinholeCamera::new(
        &[278.0, 273.0, -800.0].into(),
        &[278.0, 273.0, 0.0].into(),
        &[0.0, 1.0, 0.0].into(),
        1.0,
        37.0_f32.to_radians(),
    );

    (camera, scene)
}
