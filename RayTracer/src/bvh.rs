use crate::aabb::AABB;
use crate::hit::{HitRecord, Hittable};
use crate::hit_list::HitList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rt::random_int;
use std::cmp::Ordering;
use std::sync::Arc;
pub struct Bvh_Node {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval.min.total_cmp(&b_axis_interval.min)
}

pub fn box_compare_x(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

pub fn box_compare_y(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

pub fn box_compare_z(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl Bvh_Node {
    pub fn new(objects: Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::new(Interval::empty(), Interval::empty(), Interval::empty());
        for obj_idx in start..end {
            bbox = AABB::new_by_aabb(&bbox, &objects[obj_idx].bounding_box());
        }

        let axis = bbox.longest_axis();
        let comparator = if axis == 0 {
            box_compare_x
        } else if axis == 1 {
            box_compare_y
        } else {
            box_compare_z
        };
        let object_span = end - start;
        let left;
        let right;
        if object_span == 1 {
            left = objects[start].clone();
            right = left.clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            let mut objects = objects;
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Arc::new(Bvh_Node::new(objects.clone(), start, mid));
            right = Arc::new(Bvh_Node::new(objects, mid, end));
        }

        Self { left, right, bbox }
    }

    pub fn new_by_list(hit_list: &HitList) -> Self {
        Bvh_Node::new(hit_list.list.clone(), 0, hit_list.list.len())
    }
}

impl Hittable for Bvh_Node {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray_: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut ray_t_ref = ray_t.clone();
        if self.bbox.hit(ray_, &mut ray_t_ref) {
            let hit_left = self.left.hit(ray_, ray_t.clone(), rec);
            let hit_right = self.right.hit(ray_, ray_t.clone(), rec);
            hit_left || hit_right
        } else {
            false
        }
    }
}

unsafe impl Sync for Bvh_Node {}
