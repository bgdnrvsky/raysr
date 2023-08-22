use glam::Vec3;

pub fn unit_vec(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

pub fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let r = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

    r + (1.0 - r) * (1.0 - cosine).powi(5)
}
