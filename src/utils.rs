use glam::Vec3;

use crate::{material, sphere::Sphere};

pub fn unit_vec(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

pub fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let r = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

    r + (1.0 - r) * (1.0 - cosine).powi(5)
}
pub fn random_scene() -> Vec<Sphere> {
    let mut world = Vec::with_capacity(500);

    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material::MaterialType::Lambertian {
            albedo: Vec3::splat(0.5),
        },
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();

            let center = Vec3::new(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                world.push(if choose_mat < 0.8 {
                    Sphere::new(
                        center,
                        0.2,
                        material::MaterialType::Lambertian {
                            albedo: Vec3::new(
                                rand::random::<f32>() * rand::random::<f32>(),
                                rand::random::<f32>() * rand::random::<f32>(),
                                rand::random::<f32>() * rand::random::<f32>(),
                            ),
                        },
                    )
                } else if choose_mat < 0.95 {
                    Sphere::new(
                        center,
                        0.2,
                        material::MaterialType::Metal {
                            albedo: Vec3::new(
                                0.5 * (1.0 + rand::random::<f32>()),
                                0.5 * (1.0 + rand::random::<f32>()),
                                0.5 * (1.0 + rand::random::<f32>()),
                            ),
                            blur: 0.5 * rand::random::<f32>(),
                        },
                    )
                } else {
                    Sphere::new(
                        center,
                        0.2,
                        material::MaterialType::Dielectric {
                            refraction_index: 1.5,
                        },
                    )
                });
            }
        }
    }

    world.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material::MaterialType::Dielectric {
            refraction_index: 1.5,
        },
    ));
    world.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material::MaterialType::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    ));
    world.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material::MaterialType::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            blur: 0.0,
        },
    ));

    world
}
