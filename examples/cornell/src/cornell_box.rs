use rand::Rng;
use trace::camera::PinholeCamera;
use trace::material::Lambertian;
use trace::material::Specular;
use trace::scene::Scene;
use trace::shape::Sphere;
use trace::shape::{DynShape, Quad};
use trace::spectrum::Spectrum;

pub fn create_cornell_box<R: Rng + 'static>() -> Scene<R> {
    let floor_material = Lambertian::new(Spectrum::new([1.0, 1.0, 1.0].into()), Spectrum::zeros());
    let floor = Quad::from_points(
        [550.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 550.0],
        Box::new(floor_material),
    );

    let light_material = Lambertian::new(
        Spectrum::new([0.78, 0.78, 0.78].into()),
        Spectrum::new([4.5, 4.0, 2.0].into()),
    );
    let light = Quad::from_points(
        [343.0, 549.9, 227.0],
        [343.0, 549.9, 332.0],
        [213.0, 549.9, 332.0],
        Box::new(light_material),
    );

    let ceiling_material = Lambertian::new(Spectrum::from([255, 255, 255]), Spectrum::zeros());
    let ceiling = Quad::from_points(
        [550.0, 550.0, 0.0],
        [550.0, 550.0, 550.0],
        [0.0, 550.0, 550.0],
        Box::new(ceiling_material),
    );

    let back_wall_material = Lambertian::new(Spectrum::from([255, 255, 255]), Spectrum::zeros());
    let back_wall = Quad::from_points(
        [550.0, 0.0, 550.0],
        [0.0, 0.0, 550.0],
        [0.0, 550.0, 550.0],
        Box::new(back_wall_material),
    );

    let right_wall_material =
        Lambertian::new(Spectrum::from([255, 0, 0]), Spectrum::from([0, 0, 0]));
    let right_wall = Quad::from_points(
        [0.0, 0.0, 550.0],
        [0.0, 0.0, 0.0],
        [0.0, 550.0, 0.0],
        Box::new(right_wall_material),
    );

    let left_wall_material =
        Lambertian::new(Spectrum::from([0, 255, 0]), Spectrum::from([0, 0, 0]));
    let left_wall = Quad::from_points(
        [550.0, 0.0, 0.0],
        [550.0, 0.0, 550.0],
        [550.0, 550.0, 550.0],
        Box::new(left_wall_material),
    );

    let sphere1_material = Specular::default();
    let sphere_1 = Sphere::new(
        [294.0, 120.0, 350.0].into(),
        120.0,
        Box::new(sphere1_material),
    );

    let sphere2_material = Specular::default();
    let sphere_2 = Sphere::new(
        [400.0, 80.0, 150.0].into(),
        80.0,
        Box::new(sphere2_material),
    );

    let shapes: Vec<DynShape<R>> = vec![
        Box::new(floor),
        Box::new(ceiling),
        Box::new(back_wall),
        Box::new(left_wall),
        Box::new(right_wall),
        Box::new(sphere_1),
        Box::new(sphere_2),
        Box::new(light),
    ];

    let camera = PinholeCamera::new(
        &[278.0, 273.0, -800.0].into(),
        &[278.0, 273.0, 0.0].into(),
        &[0.0, 1.0, 0.0].into(),
        1.0,
        37.0_f32.to_radians(),
    );

    Scene::new(Box::new(camera), shapes)
}
