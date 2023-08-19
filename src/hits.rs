use crate::{material::MaterialType, ray::Ray};
use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: MaterialType,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: Default::default(),
            p: Default::default(),
            normal: Default::default(),
            material: MaterialType::Lambertian {
                albedo: Vec3::splat(0.8),
            },
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

impl<T> Hitable for Vec<T>
where
    T: Hitable,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.iter().for_each(|item| {
            if item.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        });

        hit_anything
    }
}
