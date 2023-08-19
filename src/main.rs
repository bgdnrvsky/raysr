use camera::Camera;
use glam::Vec3;
use hits::{HitRecord, Hitable};
use image::Rgb;
use sphere::Sphere;

pub mod camera;
mod ray;
mod material;
use ray::Ray;

mod hits;
mod sphere;
mod utils;

fn color<T>(ray: Ray, world: &Vec<T>) -> Vec3
where
    T: Hitable,
{
    let mut record = HitRecord::default();

    if world.hit(&ray, 0.001, f32::MAX, &mut record) {
        let target = record.p + record.normal + sphere::random_in_unit_sphere();
        0.5 * color(
            Ray::new(record.p, target - record.p),
            world,
        )
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
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    for y in (0..height).rev() {
        for x in 0..width {
            let mut col = Vec3::ZERO;

            for _ in 0..smoothing {
                let u = (x as f32 + rand::random::<f32>()) / width as f32;
                let v = (y as f32 + rand::random::<f32>()) / height as f32;
                let r = camera.get_ray(u, v);
                col += color(r, &world);
            }

            col /= smoothing as f32;
            col = Vec3::from_array(col.to_array().map(|val| val.sqrt()));

            img.put_pixel(
                x,
                y,
                Rgb(col.to_array().map(|val| (255.99 * val).floor() as u8)),
            );
        }
    }

    img.save("test.ppm").expect("Failed to save image");
}
