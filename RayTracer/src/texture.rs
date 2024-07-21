use crate::interval::Interval;
use crate::perlin::Perlin;
use crate::vec3::Vec3;
use image::{DynamicImage, GenericImageView, ImageError};
use std::path::Path;
use std::sync::Arc;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn new_by_color(&self, red: f64, blue: f64, green: f64) -> Self {
        Self::new(Vec3::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd,
            even,
        }
    }

    pub fn new_by_color(scale: f64, color1: Vec3, color2: Vec3) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(color1)),
            Arc::new(SolidColor::new(color2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let x_integer = (self.inv_scale * p.x) as i32;
        let y_integer = (self.inv_scale * p.y) as i32;
        let z_integer = (self.inv_scale * p.z) as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
        if is_even {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Result<Self, ImageError> {
        let image = image::open(Path::new(filename))?;
        Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        if self.image.height() <= 0 {
            return Vec3::new(0.0, 1.0, 0.0);
        }
        let u_ = (Interval::new(0.0, 1.0).clamp(u) * (self.image.width() - 1) as f64) as u32;
        let v_ =
            ((1.0 - Interval::new(0.0, 1.0).clamp(v)) * (self.image.height() - 1) as f64) as u32;
        let pixel = self.image.get_pixel(u_, v_);
        let color_scale = 1.0 / 255.0;
        Vec3::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }

    pub fn new_by_scale(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        0.5 * Vec3::new(1.0, 1.0, 1.0)
            * (1.0 + f64::sin(self.noise.turb(_p, 7) * 10.0 + self.scale * _p.z))
    }
}
