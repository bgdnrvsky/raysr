mod camera;
mod hits;
mod material;
mod ray;
mod sphere;
mod utils;

use camera::Camera;
use glam::Vec3;
use hits::{HitRecord, Hitable};
use image::{DynamicImage, Rgb};
use itertools::Itertools;
use material::Material;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use sphere::Sphere;
use std::sync::{Arc, Mutex};

fn color<T>(ray: Ray, world: &Vec<T>, depth: usize) -> Vec3
where
    T: Hitable,
{
    let mut record = HitRecord::default();

    if world.hit(&ray, 0.001, f32::MAX, &mut record) {
        let mut scattered = Default::default();
        let mut attenuation = Default::default();

        if depth < 50
            && record
                .material
                .scatter(&ray, &record, &mut attenuation, &mut scattered)
        {
            attenuation * color(scattered, world, depth + 1)
        } else {
            Vec3::ZERO
        }
    } else {
        let unit_direction = utils::unit_vec(ray.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::splat(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> Vec<Sphere> {
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

fn main() {
    let width = 1920;
    let height = 1080;
    let smoothing = 100;

    let img = Arc::new(Mutex::new(image::ImageBuffer::new(width, height)));

    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        width as f32 / height as f32,
        2.0,
        (lookfrom - lookat).length(),
    );

    let world = random_scene();

    eprintln!("INFO: Generated random scene!");

    (0..height)
        .rev()
        .cartesian_product(0..width)
        .par_bridge()
        .into_par_iter()
        .for_each(|(y, x)| {
            let col = std::iter::repeat_with(|| {
                let u = (x as f32 + rand::random::<f32>()) / width as f32;
                let v = (y as f32 + rand::random::<f32>()) / height as f32;
                let r = camera.get_ray(u, v);
                color(r, &world, 0)
            })
            .take(smoothing)
            .sum::<Vec3>()
                / smoothing as f32;

            img.lock().unwrap().put_pixel(
                x,
                y,
                Rgb(col
                    .to_array()
                    .map(|val| (255.99 * val.sqrt()).round() as u8)),
            );
        });

    DynamicImage::from(Arc::into_inner(img).unwrap().into_inner().unwrap())
        .rotate180()
        .save("test.ppm")
        .expect("Failed to save image");
}
