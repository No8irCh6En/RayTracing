#![allow(warnings)]
use nalgebra::Vector3;

use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct Texture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new(name: &str) -> Self {
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        u = u.max(0.0).min(1.0);
        v = v.max(0.0).min(1.0);
        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    }

    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        // 在此实现双线性插值函数, 并替换掉get_color
        // if u < 0.001 && v < 0.001 {
        //     return Vector3::new(0.0, 0.0, 0.0);
        // } else {
        //     return Vector3::new(150.0 / 255.0, 200.0 / 255.0, 50.0 / 255.0);
        // }
        // u = u - u.floor();
        // v = v - v.floor();
        u = u.max(0.01).min(0.99);
        v = v.max(0.01).min(0.99);
        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color00: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();
        let color01: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32 + 1, u_img as i32).unwrap();
        let color10: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32 + 1).unwrap();
        let color11: &VecN<u8, 3> = self
            .img_data
            .at_2d(v_img as i32 + 1, u_img as i32 + 1)
            .unwrap();

        let ratio1 = u_img - (u_img as i32) as f64;
        let ratio2 = v_img - (v_img as i32) as f64;
        let color_0 = Vector3::new(
            color00[0] as f64 * ratio1 + color10[0] as f64 * (1.0 - ratio1),
            color00[1] as f64 * ratio1 + color10[1] as f64 * (1.0 - ratio1),
            color00[2] as f64 * ratio1 + color10[2] as f64 * (1.0 - ratio1),
        );
        let color_1 = Vector3::new(
            color01[0] as f64 * ratio1 + color11[0] as f64 * (1.0 - ratio1),
            color01[1] as f64 * ratio1 + color11[1] as f64 * (1.0 - ratio1),
            color01[2] as f64 * ratio1 + color11[2] as f64 * (1.0 - ratio1),
        );
        let color = Vector3::new(
            color_0[0] as f64 * ratio2 + color_1[0] as f64 * (1.0 - ratio2),
            color_0[1] as f64 * ratio2 + color_1[1] as f64 * (1.0 - ratio2),
            color_0[2] as f64 * ratio2 + color_1[2] as f64 * (1.0 - ratio2),
        );
        Vector3::new(color[2], color[1], color[0])
    }
}
