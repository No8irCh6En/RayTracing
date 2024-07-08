#![allow(warnings)]
use crate::shader::{FragmentShaderPayload, VertexShaderPayload};
use crate::texture::Texture;
use crate::triangle::Triangle;
use nalgebra::{Matrix3, Matrix4, Vector, Vector3, Vector4};
use opencv::core::{sqrt, Mat, MatTraitConst};
use opencv::imgproc::{cvt_color, COLOR_RGB2BGR};
// use std::intrinsics::sqrtf64;
use std::os::raw::c_void;

pub type V3f = Vector3<f64>;
pub type M4f = Matrix4<f64>;

pub(crate) fn cor_dot(v1: &V3f, v2: &V3f) -> V3f {
    Vector3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

pub(crate) fn get_view_matrix(eye_pos: V3f) -> M4f {
    let mut view: Matrix4<f64> = Matrix4::identity();
    let mut up_to_down: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    view[(0, 3)] = -eye_pos[0];
    view[(1, 3)] = -eye_pos[1];
    view[(2, 3)] = -eye_pos[2];
    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64, scale: f64) -> M4f {
    let mut model: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    let angle_rad: f64 = rotation_angle.to_radians();
    model[(0, 0)] = angle_rad.cos();
    model[(0, 1)] = -angle_rad.sin();
    model[(1, 0)] = angle_rad.sin();
    model[(1, 1)] = angle_rad.cos();
    model
}

pub(crate) fn get_model_matrix_lab3(rotation_angle: f64) -> M4f {
    let mut model: M4f = Matrix4::identity();
    let rad = rotation_angle.to_radians();
    model[(0, 0)] = rad.cos();
    model[(2, 2)] = model[(0, 0)];
    model[(0, 2)] = rad.sin();
    model[(2, 0)] = -model[(0, 2)];
    let mut scale: M4f = Matrix4::identity();
    scale[(0, 0)] = 2.5;
    scale[(1, 1)] = 2.5;
    scale[(2, 2)] = 2.5;
    model * scale
}

pub(crate) fn get_projection_matrix(
    eye_fov: f64,
    aspect_ratio: f64,
    z_near: f64,
    z_far: f64,
) -> M4f {
    let mut scale: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    let fov_rad: f64 = eye_fov.to_radians();
    let t: f64 = (fov_rad / 2.0).tan() * z_near.abs();
    let r: f64 = t * aspect_ratio;
    let l: f64 = -r;
    let b: f64 = -t;
    let mut ortho: Matrix4<f64> = Matrix4::identity();
    ortho[(0, 0)] = 2.0 / (r - l);
    scale[(0, 3)] = -(r + l) / 2.0;
    ortho[(1, 1)] = 2.0 / (t - b);
    scale[(1, 3)] = -(t + b) / 2.0;
    ortho[(2, 2)] = 2.0 / (z_near - z_far);
    scale[(2, 3)] = -(z_near + z_far) / 2.0;
    let mut per_to_ortho: Matrix4<f64> = Matrix4::identity();
    per_to_ortho[(0, 0)] = z_near;
    per_to_ortho[(1, 1)] = z_near;
    per_to_ortho[(2, 2)] = z_near + z_far;
    per_to_ortho[(2, 3)] = -z_near * z_far;
    per_to_ortho[(3, 2)] = -1.0;
    per_to_ortho[(3, 3)] = 0.0;
    ortho * scale * per_to_ortho
}

pub(crate) fn get_rotation(axis: Vector3<f64>, angle: f64) -> Matrix4<f64> {
    let id: Matrix4<f64> = Matrix4::identity();
    let angle_rad: f64 = angle.to_radians();
    let mut axis_4: Matrix4<f64> = Matrix4::zeros();
    for i in 0..3 {
        for j in 0..3 {
            axis_4[(i, j)] = axis[i] * axis[j];
        }
    }
    axis_4[(3, 3)] = 1.0;
    let mut rotate_m: Matrix4<f64> = Matrix4::zeros();
    rotate_m[(0, 1)] = -axis[2];
    rotate_m[(0, 2)] = axis[1];
    rotate_m[(1, 0)] = axis[2];
    rotate_m[(1, 2)] = -axis[0];
    rotate_m[(2, 0)] = -axis[1];
    rotate_m[(2, 1)] = axis[0];
    angle_rad.cos() * id + (1.0 - angle_rad.cos()) * axis_4 + angle_rad.sin() * rotate_m
}

pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<V3f>) -> Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700,
            700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        )
        .unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image
        .convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0)
        .expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}

pub fn load_triangles(obj_file: &str) -> Vec<Triangle> {
    let (models, _) = tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).unwrap();
    let mesh = &models[0].mesh;
    let n = mesh.indices.len() / 3;
    let mut triangles = vec![Triangle::default(); n];

    // 遍历模型的每个面
    for vtx in 0..n {
        let rg = vtx * 3..vtx * 3 + 3;
        let idx: Vec<_> = mesh.indices[rg.clone()]
            .iter()
            .map(|i| *i as usize)
            .collect();

        // 记录图形每个面中连续三个顶点（小三角形）
        for j in 0..3 {
            let v = &mesh.positions[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_vertex(j, Vector4::new(v[0] as f64, v[1] as f64, v[2] as f64, 1.0));
            let ns = &mesh.normals[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_normal(j, Vector3::new(ns[0] as f64, ns[1] as f64, ns[2] as f64));
            let tex = &mesh.texcoords[2 * idx[j]..2 * idx[j] + 2];
            triangles[vtx].set_tex_coord(j, tex[0] as f64, tex[1] as f64);
        }
    }
    triangles
}

// 选择对应的Shader
pub fn choose_shader_texture(
    method: &str,
    obj_path: &str,
) -> (fn(&FragmentShaderPayload) -> Vector3<f64>, Option<Texture>) {
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = bump_fragment_shader;
    let mut tex = None;
    if method == "normal" {
        println!("Rasterizing using the normal shader");
        active_shader = normal_fragment_shader;
    } else if method == "texture" {
        println!("Rasterizing using the normal shader");
        active_shader = texture_fragment_shader;
        tex = Some(Texture::new(&(obj_path.to_owned() + "spot_texture.png")));
    } else if method == "phong" {
        println!("Rasterizing using the phong shader");
        active_shader = phong_fragment_shader;
    } else if method == "bump" {
        println!("Rasterizing using the bump shader");
        active_shader = bump_fragment_shader;
    } else if method == "displacement" {
        println!("Rasterizing using the displacement shader");
        active_shader = displacement_fragment_shader;
    }
    (active_shader, tex)
}

pub fn vertex_shader(payload: &VertexShaderPayload) -> V3f {
    payload.position
}

#[derive(Default)]
struct Light {
    pub position: V3f,
    pub intensity: V3f,
}

pub fn normal_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let result_color = (payload.normal.xyz().normalize() + Vector3::new(1.0, 1.0, 1.0)) / 2.0;
    // println!("{}", result_color*255.0);
    result_color * 255.0
}

pub fn phong_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    // 泛光、漫反射、高光系数
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    // 灯光位置和强度
    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    // ping point的信息
    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let mut result_color = Vector3::zeros(); // 保存光照结果

    // <遍历每一束光>
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular*
        // components are. Then, accumulate that result on the *result_color* object.
        let r = (light.position - point).norm();
        let light_hat = (light.position - point).normalize();
        let eye_hat = (eye_pos - point).normalize();
        let Ld =
            cor_dot(&kd, &light.intensity) / (r * r) * normal.normalize().dot(&light_hat).max(0.0);
        let h = (light_hat + eye_hat) / (light_hat + eye_hat).norm();
        let Ls = cor_dot(&ks, &light.intensity) / (r * r)
            * (normal.normalize().dot(&h).max(0.0)).powi(128);
        let La = cor_dot(&ka, &amb_light_intensity);
        result_color += (La + Ls + Ld);
    }
    result_color * 255.0
}

pub fn texture_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let texture_color: Vector3<f64> = match &payload.texture {
        // LAB3 TODO: Get the texture value at the texture coordinates of the current fragment
        // <获取材质颜色信息>
        None => Vector3::new(0.0, 0.0, 0.0),
        Some(texture) => texture.get_color_bilinear(payload.tex_coords[0], payload.tex_coords[1]), // Do modification here
    };
    let kd = texture_color / 255.0; // 材质颜色影响漫反射系数
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let color = texture_color;
    let point = payload.view_pos;
    let normal = payload.normal;

    let mut result_color = Vector3::zeros();

    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular*
        // components are. Then, accumulate that result on the *result_color* object.
        let r = (light.position - point).norm();
        let light_hat = (light.position - point).normalize();
        let eye_hat = (eye_pos - point).normalize();
        let Ld =
            cor_dot(&kd, &light.intensity) / (r * r) * normal.normalize().dot(&light_hat).max(0.0);
        let h = (light_hat + eye_hat) / (light_hat + eye_hat).norm();
        let Ls = cor_dot(&ks, &light.intensity) / (r * r)
            * (normal.normalize().dot(&h).max(0.0)).powi(128);
        let La = cor_dot(&ka, &amb_light_intensity);
        result_color += (La + Ls + Ld);
    }

    result_color * 255.0
}

pub fn bump_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement bump mapping here
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Normal n = normalize(TBN * ln)

    let tan: Vector3<f64> = Vector3::new(
        normal.x * normal.y / f64::sqrt(normal.x * normal.x + normal.z * normal.z),
        f64::sqrt(normal.x * normal.x + normal.z * normal.z),
        normal.z * normal.y / f64::sqrt(normal.x * normal.x + normal.z * normal.z),
    );
    let bump: Vector3<f64> = normal.cross(&tan);
    let TBN = Matrix3::from_columns(&[tan, bump, normal]);
    let dU = match &payload.texture {
        None => 0.0,
        Some(te) => {
            -te.get_color_bilinear(payload.tex_coords[0], payload.tex_coords[1])
                .norm()
                + te.get_color_bilinear(
                    payload.tex_coords[0] + 1.0 / te.width as f64,
                    payload.tex_coords[1],
                )
                .norm()
        }
    } * kh
        * kn;
    let dV = match &payload.texture {
        None => 0.0,
        Some(te) => {
            -te.get_color_bilinear(payload.tex_coords[0], payload.tex_coords[1])
                .norm()
                + te.get_color_bilinear(
                    payload.tex_coords[0],
                    payload.tex_coords[1] + 1.0 / te.height as f64,
                )
                .norm()
        }
    } * kh
        * kn;
    let ln = Vector3::new(-dU, -dV, 1.0);
    let normal = (TBN * ln).normalize();
    let mut result_color = Vector3::zeros();
    result_color = normal;

    result_color * 255.0
}

pub fn displacement_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement displacement mapping here
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Position p = p + kn * n * h(u,v)
    // Normal n = normalize(TBN * ln)

    let tan: Vector3<f64> = Vector3::new(
        normal.x * normal.y / f64::sqrt(normal.x * normal.x + normal.z * normal.z),
        f64::sqrt(normal.x * normal.x + normal.z * normal.z),
        normal.z * normal.y / f64::sqrt(normal.x * normal.x + normal.z * normal.z),
    );
    let bump: Vector3<f64> = normal.cross(&tan);
    let TBN = Matrix3::from_columns(&[tan, bump, normal]);
    let dU = match &payload.texture {
        None => 0.0,
        Some(te) => {
            -te.get_color_bilinear(payload.tex_coords[0], payload.tex_coords[1])
                .norm()
                + te.get_color_bilinear(
                    payload.tex_coords[0] + 1.0 / te.width as f64,
                    payload.tex_coords[1],
                )
                .norm()
        }
    } * kh
        * kn;
    let dV = match &payload.texture {
        None => 0.0,
        Some(te) => {
            -te.get_color_bilinear(payload.tex_coords[0], payload.tex_coords[1])
                .norm()
                + te.get_color_bilinear(
                    payload.tex_coords[0],
                    payload.tex_coords[1] + 1.0 / te.height as f64,
                )
                .norm()
        }
    } * kh
        * kn;
    let ln = Vector3::new(-dU, -dV, 1.0);
    let normal = (TBN * ln).normalize();
    let mut result_color = Vector3::zeros();
    let point = payload.view_pos
        + kn * normal
            * match &payload.texture {
                None => 0.0,
                Some(te) => te
                    .get_color_bilinear(payload.tex_coords[0], payload.tex_coords[1])
                    .norm(),
            };
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular*
        // components are. Then, accumulate that result on the *result_color* object.
        let r = (light.position - point).norm();
        let light_hat = (light.position - point).normalize();
        let eye_hat = (eye_pos - point).normalize();
        let Ld =
            cor_dot(&kd, &light.intensity) / (r * r) * normal.normalize().dot(&light_hat).max(0.0);
        let h = (light_hat + eye_hat) / (light_hat + eye_hat).norm();
        let Ls = cor_dot(&ks, &light.intensity) / (r * r)
            * (normal.normalize().dot(&h).max(0.0)).powi(128);
        let La = cor_dot(&ka, &amb_light_intensity);
        result_color += (La + Ls + Ld);
    }

    result_color * 255.0
}
