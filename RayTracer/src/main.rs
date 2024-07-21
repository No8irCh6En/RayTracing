mod aabb;
mod bvh;
mod camera;
mod color;
mod hit;
mod hit_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rt;
mod sphere;
mod texture;
mod vec3;
//
//
use crate::quad::gen_box;
use bvh::Bvh_Node;
use camera::Camera;
use hit_list::HitList;
use image::{ImageBuffer, Pixel, RgbImage}; //接收render传的图片，在main中文件输出
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use sphere::Sphere;
use std::sync::Arc;
use std::{fs::File, sync::WaitTimeoutResult};
use texture::{NoiseTexture, Texture};
use vec3::Vec3;
const AUTHOR: &str = "Teacher_BigN";
use crate::quad::Quad;
use crate::texture::{CheckerTexture, ImageTexture};
use rand::{self, Rng};
use rt::random_f64;
//

pub fn bouncing_spheres(path: &str) {
    let mut camera = Camera::init(1600, 16.0 / 9.0);
    let quality = 100;
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
    let checker = Arc::new(CheckerTexture::new_by_color(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new_by_tex(checker))),
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
                    let center2 = center + Vec3::new(0.0, random_f64(0.0, 0.5), 0.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        Some(sphere_material),
                    )));
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

    world = HitList::new_by_arc(Arc::new(Bvh_Node::new_by_list(&world)));

    camera.samples_per_pixel = 100;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.vfov = 20.0;
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.background = Vec3::new(0.7, 0.8, 1.0);
    camera.render(&world, &mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}

pub fn checkered_spheres(path: &str) {
    let mut camera = Camera::init(400, 16.0 / 9.0);
    let quality = 60;
    let mut img: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);

    let list = Vec::new();
    let mut world = HitList::new(list);
    let checker = Arc::new(CheckerTexture::new_by_color(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new_by_tex(checker.clone()))),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new_by_tex(checker.clone()))),
    )));
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.vfov = 20.0;
    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;
    camera.background = Vec3::new(0.7, 0.8, 1.0);
    camera.render(&world, &mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}

pub fn earth(path: &str) {
    let mut camera = Camera::init(400, 16.0 / 9.0);
    let mut img: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);
    let quality = 60;
    let earth_texture: Arc<dyn Texture> = match ImageTexture::new("input/earthmap.jpg") {
        Ok(texture) => Arc::new(texture),
        Err(_) => {
            println!("Error creating texture!");
            return;
        }
    };

    let earth_surface = Arc::new(Lambertian::new_by_tex(earth_texture));
    let globe = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Some(earth_surface),
    ));
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.lookfrom = Vec3::new(0.0, 0.0, 12.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.vfov = 20.0;
    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;
    camera.background = Vec3::new(0.7, 0.8, 1.0);
    let list = HitList::new_by_arc(globe);
    camera.render(&list, &mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}

pub fn perlin_shpere(path: &str) {
    let mut camera = Camera::init(400, 16.0 / 9.0);
    let mut img: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);
    let quality = 60;
    let list = Vec::new();
    let mut world = HitList::new(list);
    let pertext = Arc::new(NoiseTexture::new_by_scale(4.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new_by_tex(pertext.clone()))),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new_by_tex(pertext))),
    )));
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.vfov = 20.0;
    camera.defocus_angle = 0.0;
    camera.background = Vec3::new(0.7, 0.8, 1.0);
    camera.render(&world, &mut img);
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}

pub fn quads(path: &str) {
    let mut camera = Camera::init(400, 1.0);
    let mut img: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);
    let quality = 60;
    let list = Vec::new();
    let mut world = HitList::new(list);

    let left_red = Arc::new(Lambertian::new(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Vec3::new(0.2, 0.8, 0.8)));
    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 80.0;
    camera.lookfrom = Vec3::new(0.0, 0.0, 9.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.background = Vec3::new(0.7, 0.8, 1.0);
    camera.render(&mut world, &mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}

pub fn simple_light(path: &str) {
    let mut camera = Camera::init(400, 16.0 / 9.0);
    let mut img: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);
    let quality = 60;
    let list = Vec::new();
    let mut world = HitList::new(list);
    let pertext = Arc::new(NoiseTexture::new_by_scale(4.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new_by_tex(pertext.clone()))),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new_by_tex(pertext))),
    )));
    let difflight = Arc::new(DiffuseLight::new_by_color(Vec3::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Some(difflight.clone()),
    )));
    camera.samples_per_pixel = 60;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Vec3::new(26.0, 3.0, 6.0);
    camera.lookat = Vec3::new(0.0, 2.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.background = Vec3::zero();
    camera.render(&mut world, &mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}

pub fn cornel_box(path: &str) {
    let mut camera = Camera::init(600, 1.0);
    let mut img: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);
    let quality = 60;
    let list = Vec::new();
    let mut world = HitList::new(list);
    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_by_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
    world.add(gen_box(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white.clone(),
    ));
    world.add(gen_box(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white.clone(),
    ));

    camera.samples_per_pixel = 200;
    camera.max_depth = 50;
    camera.background = Vec3::new(0.0, 0.0, 0.0);
    camera.vfov = 40.0;
    camera.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    camera.lookat = Vec3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.render(&mut world, &mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
fn main() {
    let path = "output/test.jpg";

    // bouncing_spheres(path);
    // earth(path);
    // checkered_spheres(path);
    // perlin_shpere(path);
    // quads(path);
    // simple_light(path);
    cornel_box(path);
}
