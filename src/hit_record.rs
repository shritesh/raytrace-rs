use crate::{Material, Ray, Vec3};

pub struct HitRecord<'t> {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &'t Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'t> HitRecord<'t> {
    pub fn new(
        p: Vec3,
        t: f64,
        mat: &'t Material,
        r: &Ray,
        outward_normal: &Vec3,
    ) -> HitRecord<'t> {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        HitRecord {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }
}
