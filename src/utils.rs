use glam::Vec3;

pub fn unit_vec(vec: Vec3) -> Vec3 {
    vec / vec.length()
}
