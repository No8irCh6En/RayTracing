use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr_: Option<Arc<dyn Material>>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat_ptr: mat_ptr_,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray_: &Ray, int: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray_.orig;
        let a = ray_.dir.squared_length();
        let h = ray_.dir * oc;
        let c = oc.squared_length() - self.radius * self.radius;
        let disc = h * h - a * c;
        if disc < 0.0 {
            return false;
        }
        let sqrtd = f64::sqrt(disc);
        let mut root = (h - sqrtd) / a;
        if !int.surrounds(root) {
            root = (h + sqrtd) / a;
            if !int.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.point = ray_.at(rec.t);
        let out_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray_, out_normal);
        rec.mat_ptr = match &self.mat_ptr {
            Some(mat) => Some(mat.clone()),
            None => None,
        };
        true
    }
}

unsafe impl Sync for Sphere {}
