use std::sync::Arc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        mat: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
            mat_ptr: mat,
        }
    }

    pub fn set_face_normal(&mut self, ray_: &Ray, out_normal: Vec3) {
        self.front_face = ray_.dir * out_normal < 0.0;
        self.normal = if self.front_face {
            out_normal
        } else {
            -out_normal
        };
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray_: &Ray, int: Interval, rec: &mut HitRecord) -> bool;
}
