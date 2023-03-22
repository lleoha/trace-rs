use crate::math::Ray;
use crate::scene::Scene;
use crate::spectrum::Spectrum;
use image::{ImageBuffer, Rgb};
use na::Vector3;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};

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

    pub fn render(
        &self,
        scene: &Scene<ThreadRng>,
        samples: u32,
        soft_max_depth: u32,
        hard_max_depth: u32,
    ) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        let buffer = Arc::new(Mutex::new(ImageBuffer::new(self.width, self.height)));
        let counter = Arc::new(AtomicU32::new(0));

        (0..self.height).into_par_iter().for_each(|y| {
            let mut rng = ThreadRng::default();

            for x in 0..self.width {
                let mut color = Spectrum::new(Vector3::zeros());
                for _ in 0..samples {
                    let x_offset = rng.gen::<f32>() - 0.5;
                    let y_offset = rng.gen::<f32>() - 0.5;
                    let fx = ((x as f32 + 0.5 + x_offset) / (self.width as f32) - 0.5) * 2.0;
                    let fy = ((y as f32 + 0.5 + y_offset) / (self.height as f32) - 0.5) * 2.0;
                    let ray = scene.camera().sample(fx, -fy);
                    color =
                        color + self.trace(&mut rng, scene, ray, soft_max_depth, hard_max_depth);
                }
                color = color * (1.0 / samples as f32);
                buffer.lock().unwrap().put_pixel(x, y, Rgb(color.to_srgb()));
            }

            println!("{}/{}", counter.fetch_add(1, Relaxed), self.height);
        });

        // (0..self.height).par_iter_mut().for_each(|y| {

        // });

        Arc::try_unwrap(buffer).unwrap().into_inner().unwrap()
    }

    fn trace(
        &self,
        rng: &mut ThreadRng,
        scene: &Scene<ThreadRng>,
        ray: Ray,
        soft_depth_limit: u32,
        hard_depth_limit: u32,
    ) -> Spectrum {
        let mut color = Spectrum::new([0.0, 0.0, 0.0].into());
        let mut attenuation = Spectrum::new([1.0, 1.0, 1.0].into());

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

                    the_ray = Ray::new(s.point + scattered.as_ref() * 0.005, scattered);

                    // russian roulette
                    if i >= soft_depth_limit {
                        let p = attenuation.to_inner().max().clamp(0.01, 1.0);
                        if rng.gen::<f32>() > p {
                            break 'main_loop;
                        } else {
                            attenuation = attenuation * (1.0 / p);
                        }
                    }
                }
            }
        }

        color
    }
}
