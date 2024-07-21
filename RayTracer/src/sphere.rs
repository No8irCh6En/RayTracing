use crate::aabb::AABB;
use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

use std::sync::Arc;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
    pub is_moving: bool,
    pub center_v: Vec3,
    pub bbox: AABB,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr_: Option<Arc<dyn Material>>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat_ptr: mat_ptr_,
            is_moving: false,
            center_v: Vec3::zero(),
            bbox: AABB::new_by_point(
                center - Vec3::new(radius, radius, radius),
                center + Vec3::new(radius, radius, radius),
            ),
        }
    }
    pub fn new_moving(
        center: Vec3,
        center_next: Vec3,
        radius: f64,
        mat_ptr_: Option<Arc<dyn Material>>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::new_by_point(center - rvec, center + rvec);
        let box2 = AABB::new_by_point(center_next - rvec, center_next + rvec);
        let bbox = AABB::new_by_aabb(&box1, &box2);

        Self {
            center,
            radius: radius.max(0.0),
            mat_ptr: mat_ptr_,
            is_moving: true,
            center_v: (center_next - center),
            bbox: bbox,
        }
    }

    pub fn sphere_center(&self, time: f64) -> Vec3 {
        self.center + time * self.center_v
    }

    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray_: &Ray, int: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(ray_.time)
        } else {
            self.center
        };
        let oc = center - ray_.orig;
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
        Sphere::get_sphere_uv(out_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = match &self.mat_ptr {
            Some(mat) => Some(mat.clone()),
            None => None,
        };
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

unsafe impl Sync for Sphere {}
