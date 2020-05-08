use crate::{Material, Ray, Vec3};
use std::rc::Rc;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
}

impl<'t> HitRecord {
    pub fn new(
        p: Vec3,
        t: f64,
        mat: Rc<dyn Material>,
        r: &Ray,
        outward_normal: &Vec3,
    ) -> HitRecord {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        HitRecord { p, normal, mat, t }
    }
}
