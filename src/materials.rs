use crate::{utilities::random_double, HitRecord, Ray, Vec3};

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vec3, fuzz: f64) -> Self {
        Metal {
            albedo: *albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 += r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if etai_over_etat * sin_theta > 1.0 {
            unit_direction.reflect(&rec.normal)
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if random_double() < reflect_prob {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, etai_over_etat)
            }
        };

        let scattered = Ray {
            origin: rec.p,
            direction,
        };

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
