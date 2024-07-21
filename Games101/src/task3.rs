#![allow(warnings)]
pub use crate::rasterizer3::{Buffer, Rasterizer};
pub use crate::shader::FragmentShaderPayload;
pub use crate::texture::Texture;
pub use crate::utils::*;
pub use nalgebra::Vector3;
pub use opencv::core::Vector;
pub use opencv::Result;
pub use std::env;
use std::process::Command;

pub fn t3(filename: String, method: String) -> Result<()> {
    println!("选择任务3");
    let obj_file = "./models/spot/spot_triangulated_good.obj";
    // let obj_file = "./models/spot/f1-v2.obj";
    // let obj_file = "./models/spot/second.obj";
    let triangles = load_triangles(&obj_file);
    let angle = 90.0;
    let mut r = Rasterizer::new(700, 700);
    // let obj_path = "./models/spot/".to_owned();
    let obj_path = "./models/spot/".to_owned();
    // let texture_path = "F.jpg".to_owned();
    let texture_path = "spot_texture.jpg".to_owned();
    let mut tex = Texture::new(&(obj_path.clone() + &texture_path));
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = normal_fragment_shader; // 默认为<normal shader>
    let ags: Vec<String> = env::args().collect();
    println!("arg len is {}", ags.len());
    let (shader, t) = choose_shader_texture(&method, &obj_path);
    active_shader = shader;
    if let Some(tx) = t {
        tex = tx;
    }
    r.set_texture(tex);

    let eye_pos = Vector3::new(0.0, 0.0, -5.0);
    r.set_vertex_shader(vertex_shader);
    r.set_fragment_shader(active_shader);

    r.clear(Buffer::Both);
    r.set_model(get_model_matrix_lab3(angle));
    r.set_view(get_view_matrix(eye_pos));
    r.set_projection(get_projection_matrix(100.0, 1.0, -1.5, 1.5));

    r.draw(&triangles);

    let image = frame_buffer2cv_mat(r.frame_buffer());
    let v: Vector<i32> = Default::default();

    opencv::imgcodecs::imwrite(&filename, &image, &v).unwrap();

    let mut command = Command::new("feh");
    command.arg("output.png");

    // 执行命令
    match command.status() {
        Ok(status) => {
            if status.success() {
                println!("Feh executed successfully.");
            } else {
                println!("Feh exited with status: {}", status);
            }
        }
        Err(e) => {
            println!("Failed to execute feh: {}", e);
        }
    }

    Ok(())
}
