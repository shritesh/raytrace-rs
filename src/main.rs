mod vec3;

use image::ImageBuffer;
use vec3::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let i = x;
        let j = image_height - y;

        Vec3(
            i as f64 / (image_width - 1) as f64,
            j as f64 / (image_height - 1) as f64,
            0.25,
        )
        .into_rgb()
    })
    .save("image.png")
    .unwrap();
}
