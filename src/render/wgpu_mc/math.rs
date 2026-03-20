/// Conversion utilities between Leafish's cgmath types and wgpu-mc's glam types.

pub fn cgmath_vec3_to_glam(v: cgmath::Vector3<f32>) -> glam::Vec3 {
    glam::Vec3::new(v.x, v.y, v.z)
}

pub fn cgmath_point3_to_glam_vec3(p: cgmath::Point3<f64>) -> glam::Vec3 {
    glam::Vec3::new(p.x as f32, p.y as f32, p.z as f32)
}

pub fn cgmath_point3_to_glam_ivec3(p: cgmath::Point3<f64>) -> glam::IVec3 {
    glam::IVec3::new(p.x as i32, p.y as i32, p.z as i32)
}

pub fn leafish_pos_to_ivec3(pos: leafish_shared::Position) -> glam::IVec3 {
    glam::IVec3::new(pos.x, pos.y, pos.z)
}

pub fn ivec3_to_leafish_pos(v: glam::IVec3) -> leafish_shared::Position {
    leafish_shared::Position::new(v.x, v.y, v.z)
}

pub fn cgmath_mat4_to_glam(m: cgmath::Matrix4<f32>) -> glam::Mat4 {
    // cgmath and glam both use column-major layout
    let cols: [[f32; 4]; 4] = m.into();
    glam::Mat4::from_cols_array_2d(&cols)
}
