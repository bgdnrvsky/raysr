use glam::Vec3;

use crate::{
    hits::HitRecord,
    ray::{self, Ray},
    sphere, utils,
};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, blur: f32 },
    Dielectric { refraction_index: f32 },
}

impl Material for MaterialType {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialType::Lambertian { albedo } => {
                let target = record.p + record.normal + sphere::random_in_unit_sphere();
                *scattered = Ray::new(record.p, target - record.p);
                *attenuation = *albedo;

                true
            }
            Self::Metal { albedo, blur } => {
                let reflected = ray::reflect(utils::unit_vec(ray.direction()), record.normal);
                *scattered = Ray::new(
                    record.p,
                    reflected + blur.min(1.0) * sphere::random_in_unit_sphere(),
                );
                *attenuation = *albedo;

                scattered.direction().dot(record.normal) > 0.0
            }
            MaterialType::Dielectric { refraction_index } => {
                let outward_normal: Vec3;
                let reflected = ray::reflect(ray.direction(), record.normal);
                let ni_over_nt: f32;
                *attenuation = Vec3::new(1.0, 1.0, 1.0);
                let mut refracted: Vec3 = Vec3::ZERO;

                if ray.direction().dot(record.normal) > 0.0 {
                    outward_normal = -record.normal;
                    ni_over_nt = *refraction_index;
                } else {
                    outward_normal = record.normal;
                    ni_over_nt = 1.0 / *refraction_index;
                }

                if ray::refract(ray.direction(), outward_normal, ni_over_nt, &mut refracted) {
                    *scattered = Ray::new(record.p, refracted);
                    true
                } else {
                    *scattered = Ray::new(record.p, reflected);
                    false
                }
            }
        }
    }
}
