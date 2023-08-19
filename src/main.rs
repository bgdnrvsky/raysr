mod camera;
mod hits;
mod material;
mod ray;
mod sphere;
mod utils;

use camera::Camera;
use glam::Vec3;
use hits::{HitRecord, Hitable};
use image::Rgb;
use material::Material;
use ray::Ray;
use sphere::Sphere;

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

    let mut img = image::ImageBuffer::new(width, height);

    let camera = Camera::default();

    let world = vec![
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            material::MaterialType::Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.3),
            },
        ),
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            material::MaterialType::Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            },
        ),
        Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            material::MaterialType::Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
            },
        ),
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            material::MaterialType::Metal {
                albedo: Vec3::splat(0.8),
            },
        ),
    ];

    for y in (0..height).rev() {
        for x in 0..width {
            let mut col = Vec3::ZERO;

            for _ in 0..smoothing {
                let u = (x as f32 + rand::random::<f32>()) / width as f32;
                let v = (y as f32 + rand::random::<f32>()) / height as f32;
                let r = camera.get_ray(u, v);
                col += color(r, &world, 0);
            }

            col /= smoothing as f32;

            img.put_pixel(
                x,
                y,
                Rgb(col
                    .to_array()
                    .map(|val| (255.99 * val.sqrt()).floor() as u8)),
            );
        }
    }

    img.save("test.ppm").expect("Failed to save image");
}
