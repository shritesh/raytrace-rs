use crate::{utilities::random_double, HitRecord, Ray, Vec3};

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ref_idx: f64 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian { albedo } => lambertian_scatter(albedo, r_in, rec),
            Material::Metal { albedo, fuzz } => metal_scatter(albedo, *fuzz, r_in, rec),
            Material::Dielectric { ref_idx } => dielectric_scatter(*ref_idx, r_in, rec),
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 += r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn lambertian_scatter(albedo: &Vec3, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
    let scatter_direction = rec.normal + Vec3::random_unit_vector();
    let scattered = Ray {
        origin: rec.p,
        direction: scatter_direction,
    };
    Some(ScatterRecord {
        attenuation: *albedo,
        scattered,
    })
}

fn metal_scatter(albedo: &Vec3, fuzz: f64, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
    let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
    let scattered = Ray {
        origin: rec.p,
        direction: reflected + fuzz * Vec3::random_in_unit_sphere(),
    };
    if scattered.direction.dot(&rec.normal) > 0.0 {
        Some(ScatterRecord {
            attenuation: *albedo,
            scattered,
        })
    } else {
        None
    }
}

fn dielectric_scatter(ref_idx: f64, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
    let attenuation = Vec3(1.0, 1.0, 1.0);
    let etai_over_etat = if rec.front_face {
        1.0 / ref_idx
    } else {
        ref_idx
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
