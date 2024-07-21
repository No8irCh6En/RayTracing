use crate::color::write_color;
use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::random_f64;
use crate::ray::Ray;
use crate::vec3::Vec3;
use image::RgbImage;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::io::SeekFrom;
pub struct Camera {
    pub vfov: f64,
    pub width_height_ratio: f64,
    pub width: usize,
    pub height: usize,
    pub center: Vec3,
    pub pixel00: Vec3,
    pub pixel_u: Vec3,
    pub pixel_v: Vec3,
    pub samples_per_pixel: usize,
    pub max_depth: i32,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
    pub background: Vec3,
}

impl Camera {
    pub fn init(image_width: usize, image_w_h_ratio: f64) -> Self {
        let fov: f64 = 90.0;
        let image_height = (image_width as f64 / image_w_h_ratio) as usize;
        let lookfrom = Vec3::zero();
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        let defocus_angle = 0.0;
        let focus_dist = 10.0;
        let viewport_height = 2.0 * ((0.5 * fov).to_radians().tan()) * focus_dist;
        let viewport_width = viewport_height * image_w_h_ratio;
        let camera_center = lookfrom;
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let w = (lookfrom - lookat).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let pixel_u = viewport_u / image_width as f64;
        let pixel_v = viewport_v / image_height as f64;
        let viewport_layer00 = camera_center - focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_layer00 + 0.5 * (pixel_u + pixel_v);
        let defocus_radius = focus_dist * ((0.5_f64 * defocus_angle).to_radians().tan());
        Self {
            vfov: fov,
            width_height_ratio: image_w_h_ratio,
            width: image_width,
            height: image_height,
            center: camera_center,
            pixel00: pixel00,
            pixel_u: pixel_u,
            pixel_v: pixel_v,
            samples_per_pixel: 100,
            max_depth: 50,
            lookfrom: lookfrom,
            lookat: lookat,
            vup: vup,
            u: u,
            v: v,
            w: w,
            defocus_angle: defocus_angle,
            focus_dist: focus_dist,
            defocus_disk_u: defocus_radius * u,
            defocus_disk_v: defocus_radius * v,
            background: Vec3::default(),
        }
    }

    pub fn update(&mut self) {
        let viewport_height = 2.0 * ((0.5 * self.vfov).to_radians().tan()) * self.focus_dist;
        let viewport_width = viewport_height * self.width_height_ratio;
        self.center = self.lookfrom;
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(self.w).normalize();
        self.v = self.w.cross(self.u);
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;
        self.pixel_u = viewport_u / self.width as f64;
        self.pixel_v = viewport_v / self.height as f64;
        let viewport_layer =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00 = viewport_layer + 0.5 * (self.pixel_u + self.pixel_v);
        let defocus_radius = self.focus_dist * (0.5_f64 * self.defocus_angle).to_radians().tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Vec3::sample_square();
        let pixel_sample = self.pixel00
            + (i as f64 + offset.x) * self.pixel_u
            + (j as f64 + offset.y) * self.pixel_v;
        let ray_ori = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_ori;
        let ray_time = random_f64(0.0, 1.0);
        Ray::new(ray_ori, ray_dir, ray_time)
    }

    pub fn ray_color<T: Hittable>(&self, ray_: Ray, world: &T, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, None);
        if !world.hit(&ray_, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background.clone();
        }
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let mut attenuation = Vec3::zero();
        let color_from_emission = rec
            .mat_ptr
            .as_ref()
            .unwrap()
            .emitted(rec.u, rec.v, rec.point);
        if rec
            .mat_ptr
            .as_ref()
            .unwrap()
            .scatter(&ray_, &rec, &mut attenuation, &mut scattered)
        {
            let color_from_scatter =
                attenuation.cor_dot(self.ray_color(scattered, world, depth - 1));
            return color_from_emission + color_from_scatter;
        }
        color_from_emission
    }

    pub fn render<T: Hittable + Sync>(&mut self, world: &T, img: &mut RgbImage) {
        self.update();
        let bar: ProgressBar = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.height * self.width) as u64)
        };
        let pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        for i in 0..self.width {
            for j in 0..self.height {
                let pixel_color = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| self.ray_color(self.get_ray(i, j), world, self.max_depth))
                    .sum::<Vec3>();
                write_color(pixel_color * pixel_sample_scale, img, i, j);
                bar.inc(1);
            }
        }
        bar.finish();
    }

    pub fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}
