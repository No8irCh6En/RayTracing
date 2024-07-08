use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;
pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        scattered.orig = rec.point;
        let mut dir = rec.normal + Vec3::random_unit_vector();
        if dir.near_zero() {
            dir = rec.normal;
        }
        scattered.dir = dir;
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz_ = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self {
            albedo,
            fuzz: fuzz_,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray_in.dir, rec.normal).normalize()
            + self.fuzz * Vec3::random_unit_vector();
        scattered.orig = rec.point;
        scattered.dir = reflected;
        *attenuation = self.albedo;
        scattered.dir * rec.normal > 0.0
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
    pub fn reflectance(cosine: f64, refr_idx: f64) -> f64 {
        let mut r0 = (1.0 - refr_idx) / (1.0 + refr_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut rng = rand::thread_rng();
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_dir = ray_in.dir.normalize();
        let cos_theta = (-unit_dir * rec.normal).min(1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = ri * sin_theta > 1.0;
        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rng.gen_range(0.0..1.0) {
            let reflected = Vec3::reflect(unit_dir, rec.normal);
            scattered.dir = reflected;
        } else {
            let refracted = Vec3::refract(unit_dir, rec.normal, ri);
            scattered.dir = refracted;
        }
        scattered.orig = rec.point;
        true
    }
}
