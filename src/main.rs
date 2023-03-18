use image::{Rgb as ImageRgb, RgbImage};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::error::Error;
use std::path::Path;
use tracers::cornell_box::create_cornell_box;
use tracers::renderer::Renderer;
use tracers::spectrum::Spectrum;

fn main() -> Result<(), Box<dyn Error>> {

    let width = 512;
    let height = 512;
    let mut rng = SmallRng::from_entropy();
    let cornell_box = create_cornell_box();
    let renderer = Renderer::new(width, height, Spectrum::from_rgb([0, 0, 0]));
    let buffer = renderer.render(&mut rng, &cornell_box.0, &cornell_box.1, 10000, 100);

    let mut image = RgbImage::new(width, height);
    let pixels = buffer.as_slice();
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels.get((y * width + x) as usize).unwrap();
            image.put_pixel(x, y, ImageRgb::from(pixel.to_rgb()));
        }
    }
    image.save(Path::new("cornell.png")).unwrap();

    Ok(())
}
