use crate::aabb::AABB;
use crate::hit::{HitRecord, Hittable};
use crate::hit_list::HitList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Deref;
use std::sync::Arc;
pub struct Quad {
    Q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    D: f64,
}

impl Quad {
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();
        let D = normal * Q;
        let w = n / (n * n);
        //set_bounding_box
        let bbox_diag1 = AABB::new_by_point(Q, Q + u + v);
        let bbox_diag2 = AABB::new_by_point(Q + u, Q + v);
        let bbox = AABB::new_by_aabb(&bbox_diag1, &bbox_diag2);
        Self {
            Q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            D,
        }
    }

    pub fn is_interior(alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
        let unit_inteveral = Interval::new(0.0, 1.0);
        if !unit_inteveral.contains(alpha) || !unit_inteveral.contains(beta) {
            return false;
        }
        rec.u = alpha;
        rec.v = beta;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal * ray.dir;
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.normal * (self.Q - ray.orig)) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = self.w * (planar_hitpt_vector.cross(self.v));
        let beta = self.w * (self.u.cross(planar_hitpt_vector));
        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.point = intersection;
        rec.mat_ptr = Some(self.mat.clone());
        rec.set_face_normal(ray, self.normal.clone());
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

unsafe impl Sync for Quad {}

pub fn gen_box(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let list = Vec::new();
    let mut sides: HitList = HitList::new(list);
    let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);
    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    )));

    sides.add(Arc::new(Quad::new(
        Vec3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    )));

    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    )));

    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    )));

    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    )));
    Arc::new(sides)
}
