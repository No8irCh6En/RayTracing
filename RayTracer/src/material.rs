use std::sync::Arc;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::texture::{CheckerTexture, SolidColor, Texture};
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
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct Lambertian {
    pub tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_by_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
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
        let mut dir = rec.normal + Vec3::random_unit_vector();
        if dir.near_zero() {
            dir = rec.normal;
        }
        *scattered = Ray::new(rec.point, dir, ray_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, rec.point);
        true
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::zero()
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
        *scattered = Ray::new(rec.point, reflected, ray_in.time);
        *attenuation = self.albedo;
        scattered.dir * rec.normal > 0.0
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::zero()
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
            *scattered = Ray::new(rec.point, reflected, ray_in.time);
        } else {
            let refracted = Vec3::refract(unit_dir, rec.normal, ri);
            *scattered = Ray::new(rec.point, refracted, ray_in.time);
        }
        true
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
    pub fn new_by_color(emit: Vec3) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.tex.value(u, v, p)
    }
}
