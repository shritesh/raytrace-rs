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
use utilities::{random_double, random_range};
use vec3::Vec3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
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

fn random_scene() -> HittableList {
    let mut objects = Vec::<Box<dyn Hittable>>::new();

    objects.push(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Rc::new(Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Rc<dyn Material> = if choose_mat < 0.8 {
                    // Diffuse
                    Rc::new(Lambertian {
                        albedo: Vec3::random() * Vec3::random(),
                    })
                } else if choose_mat < 0.95 {
                    Rc::new(Metal {
                        albedo: Vec3::random_in_range(0.5, 1.0),
                        fuzz: random_range(0.0, 0.5),
                    })
                } else {
                    // glass
                    Rc::new(Dielectric { ref_idx: 1.5 })
                };

                objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    mat,
                }))
            }
        }
    }

    objects.push(Box::new(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Rc::new(Dielectric { ref_idx: 1.5 }),
    }));

    objects.push(Box::new(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Rc::new(Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        }),
    }));

    objects.push(Box::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Rc::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    objects
}

pub fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let world = random_scene();

    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
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
