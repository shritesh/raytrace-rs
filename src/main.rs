pub mod camera;
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod materials;
pub mod ray;
pub mod sphere;
pub mod utilities;
pub mod vec3;

use camera::Camera;
use hit_record::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use image::ImageBuffer;
use materials::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use utilities::random_double;
use vec3::Vec3;

pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(sr) = rec.mat.scatter(r, &rec) {
            sr.attenuation * ray_color(&sr.scattered, world, depth - 1)
        } else {
            Vec3(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
    }
}

pub fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 100;

    let world = HittableList(vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            mat: Rc::new(Lambertian {
                albedo: Vec3(0.1, 0.2, 0.5),
            }),
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            mat: Rc::new(Lambertian {
                albedo: Vec3(0.8, 0.8, 0.0),
            }),
        }),
        Box::new(Sphere {
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Rc::new(Metal {
                albedo: Vec3(0.8, 0.6, 0.2),
                fuzz: 0.3,
            }),
        }),
        Box::new(Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Rc::new(Dielectric { ref_idx: 1.5 }),
        }),
        Box::new(Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: -0.45,
            mat: Rc::new(Dielectric { ref_idx: 1.5 }),
        }),
    ]);

    let cam = Camera::new(
        Vec3(-2.0, 2.0, 1.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let i = x;
        let j = image_height - y;

        let mut pixel_color = Vec3(0.0, 0.0, 0.0);

        for _ in 0..samples_per_pixel {
            let u = (i as f64 + random_double()) / (image_width - 1) as f64;
            let v = (j as f64 + random_double()) / (image_height - 1) as f64;
            let r = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &world, max_depth);
        }

        pixel_color.into_rgb(samples_per_pixel)
    })
    .save("image.png")
    .unwrap();
}
