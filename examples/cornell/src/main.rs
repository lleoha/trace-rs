mod cornell_box;
mod data;

use crate::cornell_box::create_cornell_box;

use image::buffer::ConvertBuffer;
use image::{ImageBuffer, Rgb};
use std::error::Error;
use std::path::Path;
use std::time::Instant;
use trace::color::spectrum::Spectrum;
use trace::renderer::Renderer;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 512;
    let height = 512;
    let cornell_box = create_cornell_box();
    let renderer = Renderer::new(width, height, Spectrum::black());

    let start = Instant::now();
    let image = renderer.render(&cornell_box, 10_000, 10, 50);
    let duration = Instant::now() - start;
    println!("Rendering time: {}s.", duration.as_secs_f32());

    let image_rgb: ImageBuffer<Rgb<u8>, _> = image.convert();
    image_rgb.save(Path::new("./cornell.png"))?;

    Ok(())
}
