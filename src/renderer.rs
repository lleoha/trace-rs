use crate::camera::Camera;
use crate::math::ray::Ray;
use crate::scene::Scene;
use crate::spectrum::Spectrum;
use na::Vector3;
use rand::Rng;

pub struct Renderer {
    width: u32,
    height: u32,
    background_color: Spectrum,
}

impl Renderer {
    pub fn new(width: u32, height: u32, background_color: Spectrum) -> Self {
        Self {
            width,
            height,
            background_color,
        }
    }

    pub fn render<R: Rng>(
        &self,
        rng: &mut R,
        camera: &(impl Camera + ?Sized),
        scene: &Scene<R>,
        samples: u32,
        max_depth: u32,
    ) -> Vec<Spectrum> {
        let mut buffer = Vec::new();

        for y in 0..self.height {
            dbg!(y);
            for x in 0..self.width {
                let mut color = Spectrum::new(Vector3::zeros());
                for _i in 0..samples {
                    let x_offset = rng.gen::<f32>() - 0.5;
                    let y_offset = rng.gen::<f32>() - 0.5;
                    let fx = ((x as f32 + 0.5 + x_offset) / (self.width as f32) - 0.5) * 2.0;
                    let fy = ((y as f32 + 0.5 + y_offset) / (self.height as f32) - 0.5) * 2.0;
                    let ray = camera.sample(fx, -fy);
                    color = color + self.trace(rng, scene, ray, max_depth / 2, max_depth);
                }
                color = color * (1.0 / samples as f32);
                buffer.push(color);
            }
        }

        buffer
    }

    fn trace<R: Rng>(
        &self,
        rng: &mut R,
        scene: &Scene<R>,
        ray: Ray,
        soft_depth_limit: u32,
        hard_depth_limit: u32,
    ) -> Spectrum {
        let mut color = Spectrum::new([0.0, 0.0, 0.0]);
        let mut attenuation = Spectrum::new([1.0, 1.0, 1.0]);

        let mut the_ray = ray;
        'main_loop: for i in 0..hard_depth_limit {
            let intersection = scene.intersect(&the_ray);
            match intersection {
                None => {
                    color = color + (attenuation * self.background_color);
                    break;
                }
                Some(ref s) => {
                    let material = s.shape.material();
                    let scattered = material.scatter(s, the_ray.direction(), rng);

                    color =
                        color + material.emission(s, the_ray.direction(), &scattered) * attenuation;
                    attenuation =
                        attenuation * material.attenuation(s, the_ray.direction(), &scattered);

                    the_ray = Ray::new(s.point + &scattered.into_inner() * 0.005, scattered);

                    // russian roulette
                    if i >= soft_depth_limit {
                        let p = attenuation.to_inner().max().clamp(0.0, 1.0);
                        if rng.gen::<f32>() > p {
                            break 'main_loop;
                        }
                        attenuation = attenuation * (1.0 / p);
                    }
                }
            }
        }

        color
    }
}
