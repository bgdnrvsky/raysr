use std::f32::consts::PI;

use glam::Vec3;

use crate::{ray::Ray, utils};

#[allow(unused)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        // vfov is top to bottom in degrees
        let theta = vfov * PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;

        let w = utils::unit_vec(lookfrom - lookat);
        let u = utils::unit_vec(vup.cross(w));
        let v = w.cross(u);

        Self {
            origin: lookfrom,
            lower_left: lookfrom
                - half_width * u * focus_dist
                - half_height * v * focus_dist
                - focus_dist * w,
            horizontal: 2.0 * half_width * u * focus_dist,
            vertical: 2.0 * half_height * v * focus_dist,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0)
            - Vec3::new(1.0, 1.0, 0.0);

        if p.dot(p) < 1.0 {
            break p;
        }
    }
}
