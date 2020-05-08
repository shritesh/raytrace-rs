use crate::{HitRecord, Hittable, Material, Ray, Vec3};
use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            for chosen_root in [-root, root].iter() {
                let temp = (-half_b + chosen_root) / a;
                if temp < t_max && temp > t_min {
                    let p = r.at(temp);
                    let outward_normal = (p - self.center) / self.radius;
                    return Some(HitRecord::new(
                        p,
                        temp,
                        self.mat.clone(),
                        r,
                        &outward_normal,
                    ));
                }
            }
        }

        None
    }
}
