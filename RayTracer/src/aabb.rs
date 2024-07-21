use std::ops::Index;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x_: Interval, y_: Interval, z_: Interval) -> Self {
        let delta = 0.0001;
        let mut x = x_;
        let mut y = y_;
        let mut z = z_;
        if x.size() < delta {
            x = x.expand(delta);
        }
        if y.size() < delta {
            y = y.expand(delta);
        }
        if z.size() < delta {
            z = z.expand(delta);
        }
        Self { x, y, z }
    }

    pub fn new_by_point(a: Vec3, b: Vec3) -> Self {
        let mut x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let mut y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let mut z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };
        let delta = 0.0001;
        if x.size() < delta {
            x = x.expand(delta);
        }
        if y.size() < delta {
            y = y.expand(delta);
        }
        if z.size() < delta {
            z = z.expand(delta);
        }
        Self { x, y, z }
    }

    pub fn new_by_aabb(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::new_by_interval(box0.x, box1.x),
            y: Interval::new_by_interval(box0.y, box1.y),
            z: Interval::new_by_interval(box0.z, box1.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = ray.orig;
        let ray_dir = ray.dir;
        for axis in 0..3 {
            let inter = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.index(axis as usize);
            let t0 = (inter.min - ray_orig.index(axis as usize)) * adinv;
            let t1 = (inter.max - ray_orig.index(axis as usize)) * adinv;
            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() < self.y.size() {
            return if self.x.size() < self.z.size() { 0 } else { 2 };
        }
        if self.y.size() < self.z.size() {
            1
        } else {
            2
        }
    }
}
