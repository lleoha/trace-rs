use num_complex::Complex;
use rand::Rng;
use trace::camera::ThinLensCamera;
use trace::color::spectrum::Spectrum;
use trace::material::{Lambertian, Metal, Translucent};
use trace::scene::Scene;
use trace::shape::{Quad, Sphere};

pub fn create_scene<R: Rng + 'static>() -> Scene<R> {
    let floor_material = Lambertian::new(Spectrum::from_srgb(&[0.5, 1.0, 0.3].into()), Spectrum::black());
    let floor = Quad::from_points(
        [-1000.0, 0.0, -1000.0],
        [-1000.0, 0.0, 1000.0],
        [1000.0, 0.0, 1000.0],
        floor_material,
    );

    let light_material = Lambertian::new(Spectrum::black(), Spectrum::from_linear_rgb(&[1.5, 1.5, 1.5].into()));
    let light = Quad::new(
        [-10.0, 10.0, -5.0].into(),
        [10.0, 10.0, -15.0].into(),
        [10.0, 10.0, 15.0].into(),
        light_material,
    );

    let camera = ThinLensCamera::new(
        &[0.0, 1.5, -5.0].into(),
        &[0.0, 0.7, 0.0].into(),
        &[0.0, 1.0, 0.0].into(),
        1920.0 / 1080.0,
        60.0_f32.to_radians(),
        0.4,
        5.0,
    );

    let matte = Lambertian::new(Spectrum::from_srgb_u8(&[240, 100, 0].into()), Spectrum::black());
    let matte_ball = Sphere::new([-2.0, 0.71, 1.5].into(), 0.7, matte);

    let translucent = Translucent::new(1.5168);
    let silver_ball = Sphere::new([0.0, 0.71, 0.0].into(), 0.7, translucent);

    let silver = Metal::new(Complex::new(0.051585, 3.9046));
    let translucent_ball = Sphere::new([2.0, 0.71, -1.5].into(), 0.7, silver);

    let mut scene = Scene::new(camera);
    scene
        .add(floor)
        .add(light)
        .add(matte_ball)
        .add(silver_ball)
        .add(translucent_ball);

    scene
}
