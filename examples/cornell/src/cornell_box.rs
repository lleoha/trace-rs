use num_complex::Complex;
use rand::Rng;
use trace::camera::PinholeCamera;
use trace::material::{Lambertian, Metal};

use trace::scene::Scene;
use trace::shape::Quad;
use trace::shape::Sphere;
use trace::spectrum::Spectrum;

pub fn create_cornell_box<R: Rng + 'static>() -> Scene<R> {
    let floor_material = Lambertian::new(Spectrum::new([1.0, 1.0, 1.0].into()), Spectrum::zeros());
    let floor = Quad::from_points(
        [550.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 550.0],
        floor_material,
    );

    let light_material = Lambertian::new(
        Spectrum::new([0.78, 0.78, 0.78].into()),
        Spectrum::new([5.5, 5.0, 2.5].into()),
    );
    let light = Quad::from_points(
        [343.0, 549.9, 227.0],
        [343.0, 549.9, 332.0],
        [213.0, 549.9, 332.0],
        light_material,
    );

    let ceiling_material = Lambertian::new(Spectrum::from([255, 255, 255]), Spectrum::zeros());
    let ceiling = Quad::from_points(
        [550.0, 550.0, 0.0],
        [550.0, 550.0, 550.0],
        [0.0, 550.0, 550.0],
        ceiling_material,
    );

    let back_wall_material = Lambertian::new(Spectrum::from([255, 255, 255]), Spectrum::zeros());
    let back_wall = Quad::from_points(
        [550.0, 0.0, 550.0],
        [0.0, 0.0, 550.0],
        [0.0, 550.0, 550.0],
        back_wall_material,
    );

    let right_wall_material =
        Lambertian::new(Spectrum::from([200, 0, 0]), Spectrum::from([0, 0, 0]));
    let right_wall = Quad::from_points(
        [0.0, 0.0, 550.0],
        [0.0, 0.0, 0.0],
        [0.0, 550.0, 0.0],
        right_wall_material,
    );

    let left_wall_material =
        Lambertian::new(Spectrum::from([0, 200, 0]), Spectrum::from([0, 0, 0]));
    let left_wall = Quad::from_points(
        [550.0, 0.0, 0.0],
        [550.0, 0.0, 550.0],
        [550.0, 550.0, 550.0],
        left_wall_material,
    );

    let sphere1_silver = Metal::new(Complex::new(0.051585, 3.9046));
    let sphere_1 = Sphere::new([294.0, 120.0, 350.0].into(), 120.0, sphere1_silver);

    let sphere2_silver = Metal::new(Complex::new(0.051585, 3.9046));
    let sphere_2 = Sphere::new([400.0, 80.0, 150.0].into(), 80.0, sphere2_silver);

    let camera = PinholeCamera::new(
        &[278.0, 273.0, -800.0].into(),
        &[278.0, 273.0, 0.0].into(),
        &[0.0, 1.0, 0.0].into(),
        1.0,
        37.0_f32.to_radians(),
    );

    let mut scene = Scene::new(camera);
    scene
        .add(floor)
        .add(ceiling)
        .add(back_wall)
        .add(left_wall)
        .add(right_wall)
        .add(sphere_1)
        .add(sphere_2)
        .add(light);

    scene
}
