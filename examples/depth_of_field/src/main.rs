mod dof_scene;

use crate::dof_scene::create_scene;
use image::buffer::ConvertBuffer;
use image::{ImageBuffer, Rgb};
use std::error::Error;
use std::path::Path;
use std::time::Instant;
use trace::color::spectrum::Spectrum;
use trace::renderer::Renderer;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 1920 / 2;
    let height = 1080 / 2;
    let scene = create_scene();
    let renderer = Renderer::new(width, height, Spectrum::from_srgb(&[0.01, 0.05, 0.1].into()));

    let start = Instant::now();
    let image = renderer.render(&scene, 10_000, 10, 50);
    let duration = Instant::now() - start;
    println!("Rendering time: {}s.", duration.as_secs_f32());

    let image_rgb: ImageBuffer<Rgb<u8>, _> = image.convert();
    image_rgb.save(Path::new("dof_10k.png"))?;

    Ok(())
}
