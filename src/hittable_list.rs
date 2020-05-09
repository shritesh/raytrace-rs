use crate::{HitRecord, Hittable, Ray, Sphere};

pub type HittableList = Vec<Sphere>;

impl<'t> Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        let mut rec = None;

        for obj in self {
            if let Some(temp_rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        rec
    }
}
