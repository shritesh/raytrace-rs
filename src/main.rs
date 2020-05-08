pub mod ray;
pub mod vec3;

use image::ImageBuffer;
use ray::Ray;
use vec3::Vec3;

pub fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

pub fn main() {
    let image_width = 256;
    let image_height = 256;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.25, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, 1.0);

    ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let i = x;
        let j = image_height - y;

        let u = i as f64 / (image_width - 1) as f64;
        let v = j as f64 / (image_height - 1) as f64;

        let r = Ray {
            origin,
            direction: lower_left_corner + u * horizontal + v * vertical,
        };

        ray_color(&r).into_rgb()
    })
    .save("image.png")
    .unwrap();
}
