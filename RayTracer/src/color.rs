use crate::interval::Interval;
use crate::vec3::Vec3;
use image::RgbImage;

pub fn linear_to_gamma(x: f64) -> f64 {
    if x > 0.0 {
        return f64::sqrt(x);
    }
    0.0
}
/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    let int = Interval::new(0.0, 0.999);
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);
    let convert_color = [
        (256.0 * int.clamp(r)) as u8,
        (256.0 * int.clamp(g)) as u8,
        (256.0 * int.clamp(b)) as u8,
    ];
    *pixel = image::Rgb(convert_color);
}
