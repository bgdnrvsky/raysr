use crate::hits::Hitable;
use glam::Vec3;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p;

    loop {
        p = 2.0 * Vec3::new(rand::random(), rand::random(), rand::random()) - Vec3::ONE;

        if p.length_squared() < 1.0 {
            break;
        }
    }

    p
}

impl Hitable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        record: &mut crate::hits::HitRecord,
    ) -> bool {
        let oc = ray.origin() - self.center;

        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let temp = (-1.0 * b - discriminant.sqrt()) / a;

            if (t_min..t_max).contains(&temp) {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true;
            }

            let temp = (-1.0 * b + discriminant.sqrt()) / a;
            if (t_min..t_max).contains(&temp) {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}
