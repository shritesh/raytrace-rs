use image::{ImageBuffer, Rgb};

fn main() {
    let image_width = 256;
    let image_height = 256;

    ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let i = x;
        let j = image_height - y;

        let r = i as f64 / (image_width - 1) as f64;
        let g = j as f64 / (image_height - 1) as f64;
        let b = 0.25;

        Rgb([
            (r * 255.999) as u8,
            (g * 255.999) as u8,
            (b * 255.999) as u8,
        ])
    })
    .save("image.png")
    .unwrap();
}
