mod camera;
mod hits;
mod material;
mod ray;
mod sphere;
mod utils;

use camera::Camera;
use clap::Parser;
use glam::Vec3;
use hits::{HitRecord, Hitable};
use image::{DynamicImage, Rgb};
use itertools::Itertools;
use loading::Loading;
use material::Material;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

#[derive(Debug, Parser)]
#[command(name = "raysr", version)]
#[command(about = "Ray tracer written while following 'Ray tracer in a weekend' book!", long_about=None)]
#[command(arg_required_else_help = true)]
struct Args {
    #[arg(short, long, value_name = "NON-NEGATIVE")]
    width: u32,
    #[arg(short, long, value_name = "NON-NEGATIVE")]
    height: u32,
    #[arg(short, long, default_value_t = 100, value_name = "NON-NEGATIVE")]
    smoothing: usize,
    #[arg(short, long, value_name = "FILE")]
    out: String,
    #[arg(
        long,
        default_value_t = false,
        help = "Open generated scene afterwards"
    )]
    open: bool,
}

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
    let args = Args::parse();

    let img = Arc::new(Mutex::new(image::ImageBuffer::new(args.width, args.height)));

    let lookfrom = Vec3::new(3.5, 1.0, 2.5);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        110.0,
        args.width as f32 / args.height as f32,
        0.0,
        (lookfrom - lookat).length(),
    );

    let world = utils::random_scene();

    let progress = Arc::new(Mutex::new(Loading::default()));
    progress.lock().unwrap().info("Generated random scene!");
    let start = Instant::now();

    (0..args.height)
        .rev()
        .cartesian_product(0..args.width)
        .enumerate()
        .par_bridge()
        .into_par_iter()
        .for_each(|(i, (y, x))| {
            progress.lock().unwrap().text(format!(
                "Rendering '{output}'... {curr}/{total} px",
                output = args.out,
                curr = i,
                total = args.width * args.height
            ));

            let col = std::iter::repeat_with(|| {
                let u = (x as f32 + rand::random::<f32>()) / args.width as f32;
                let v = (y as f32 + rand::random::<f32>()) / args.height as f32;
                let r = camera.get_ray(u, v);
                color(r, &world, 0)
            })
            .take(args.smoothing)
            .sum::<Vec3>()
                / args.smoothing as f32;

            img.lock().unwrap().put_pixel(
                x,
                y,
                Rgb(col
                    .to_array()
                    .map(|val| (255.99 * val.sqrt()).round() as u8)),
            );
        });

    let progress = Arc::try_unwrap(progress).unwrap().into_inner().unwrap();

    let generated = match DynamicImage::from(Arc::into_inner(img).unwrap().into_inner().unwrap())
        .rotate180()
        .save(&args.out)
    {
        Ok(_) => {
            progress.success(format!(
                "Successfully generated scene in {elapsed} seconds",
                elapsed = start.elapsed().as_secs_f32()
            ));
            true
        }
        Err(_) => {
            progress.fail("Failed to generate scene");
            false
        }
    };

    progress.end();

    if generated && args.open {
        open::that(args.out).expect("Failed to open resulting scene");
    }
}
