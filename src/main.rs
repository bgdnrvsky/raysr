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

fn main() {
    let width = 1920;
    let height = 1080;
    let smoothing = 100;

    let img = Arc::new(Mutex::new(image::ImageBuffer::new(width, height)));

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        25.0,
        width as f32 / height as f32,
    );

    let world = vec![
        Sphere::new(
            // Ball in the center
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            material::MaterialType::Lambertian {
                albedo: Vec3::new(0.1, 0.2, 0.5),
            },
        ),
        Sphere::new(
            // Ball on the bottom
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            material::MaterialType::Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            },
        ),
        Sphere::new(
            // Ball on the left
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            material::MaterialType::Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                blur: 1.0,
            },
        ),
        Sphere::new(
            // Ball on the right
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            material::MaterialType::Dielectric {
                refraction_index: 1.5,
            },
        ),
        Sphere::new(
            // Ball on the right (inner)
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            material::MaterialType::Dielectric {
                refraction_index: 1.5,
            },
        ),
    ];

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
