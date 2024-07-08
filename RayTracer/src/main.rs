mod camera;
mod color;
mod hit;
mod hit_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;
//
//
use camera::Camera;
use hit_list::HitList;
use image::{ImageBuffer, Pixel, RgbImage}; //接收render传的图片，在main中文件输出
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use std::fs::File;
use std::sync::Arc;
use vec3::Vec3;
const AUTHOR: &str = "Teacher_BigN";
use rand::{self, Rng};

//

fn random_f64(a: f64, b: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(a..b)
}

fn main() {
    let path = "output/test.jpg";
    let mut camera = Camera::init(1200, 16.0 / 9.0);
    let quality = 60;
    let mut img: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);
    //
    let list = Vec::new();
    let mut world = HitList::new(list);
    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(ground_material),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64(0.0, 1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * random_f64(0.0, 1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let albedo: Vec3;
                if choose_mat < 0.8 {
                    albedo = Vec3::new(
                        random_f64(0.0, 1.0) * random_f64(0.0, 1.0),
                        random_f64(0.0, 1.0) * random_f64(0.0, 1.0),
                        random_f64(0.0, 1.0) * random_f64(0.0, 1.0),
                    );

                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                } else if choose_mat < 0.95 {
                    albedo = Vec3::new(
                        random_f64(0.5, 1.0),
                        random_f64(0.5, 1.0),
                        random_f64(0.5, 1.0),
                    );
                    let fuzz = random_f64(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Some(material1),
    )));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Some(material2),
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Some(material3),
    )));

    camera.samples_per_pixel = 500;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.vfov = 20.0;
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.render(&world, &mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
