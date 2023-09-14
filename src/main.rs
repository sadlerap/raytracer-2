use std::{
    io::{BufWriter, Result, Write},
    path::PathBuf, iter,
};

use camera::CameraBuilder;
use clap::{Parser, ValueEnum};
use material::{Dielectric, Material, Metal};
use rand::{thread_rng, Rng};
use vec3::{Color, Vec3};

use crate::{
    geometry::{HittableList, Sphere},
    material::Lambertian,
    vec3::Point3,
};

pub mod camera;
pub mod geometry;
mod material;
pub mod ray;
mod util;
pub mod vec3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long)]
    scene: Scene,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Scene {
    Spheres,
    BookCover,
}

fn spheres<W: Write>(output: &mut W) -> Result<()> {
    // Materials

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // World

    let ground_sphere = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    let center_sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &material_center);
    let left_sphere = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    let left_inner_sphere = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, &material_left);
    let right_sphere = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, &material_right);

    let mut world = HittableList::default();
    world.add(&ground_sphere);
    world.add(&center_sphere);
    world.add(&left_sphere);
    world.add(&left_inner_sphere);
    world.add(&right_sphere);

    // Camera

    let camera = camera::CameraBuilder::default()
        .with_image_width(1920)
        .with_aspect_ratio(16.0 / 9.0)
        .with_samples_per_pixel(1000)
        .with_recursion_depth(50)
        .with_vertical_field_of_view(90.0)
        .look_from(Point3::new(-2.0, 2.0, 1.0))
        .look_at(Point3::new(0.0, 0.0, -1.0))
        .with_up(Vec3::new(0.0, 1.0, 0.0))
        .build();

    // Render

    camera.render_to_io(&world, output)?;
    drop(world);

    Ok(())
}

fn book_cover<W: Write>(output: &mut W) -> Result<()> {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let mut spheres = Vec::with_capacity(22 * 22 + 3);

    let mut sphere_materials: Vec<Box<dyn Material>> = (-11..11)
        .flat_map(|i| (-11..11).map(move |j| (i, j)))
        .map(|(i, j)| {
            let choose_mat: f32 = rand::random();
            let center = Point3::new(
                i as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                j as f32 + 0.9 * rand::random::<f32>(),
            );

            (choose_mat, center)
        })
        .filter(|(_, center)| (*center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9)
        .map(|(choose_mat, center)| {
            spheres.push((center, 0.2));
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::random() * Color::random();
                let material = Lambertian::new(albedo);
                let material = Box::new(material) as Box<dyn Material>;

                material
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::random_in_range(0.5, 1.0);
                let fuzz = thread_rng().gen_range(0.0..0.5);
                let material = Metal::new(albedo, fuzz);
                let material = Box::new(material) as Box<dyn Material>;

                material
            } else {
                // glass
                let material = Dielectric::new(1.5);
                let material = Box::new(material) as Box<dyn Material>;

                material
            }
        })
        .collect();

    sphere_materials.push(Box::new(Dielectric::new(1.5)));
    sphere_materials.push(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    sphere_materials.push(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));

    spheres.push((Point3::new(0.0, 1.0, 0.0), 1.0));
    spheres.push((Point3::new(-4.0, 1.0, 0.0), 1.0));
    spheres.push((Point3::new(4.0, 1.0, 0.0), 1.0));

    let spheres: Vec<Sphere> = iter::once(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ))
    .chain(
        spheres
            .iter()
            .zip(sphere_materials.iter())
            .map(|((center, radius), material)| Sphere::new(*center, *radius, material.as_ref())),
    )
    .collect();

    for sphere in spheres.iter() {
        world.add(sphere)
    }

    let camera = CameraBuilder::default()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(1920)
        .with_samples_per_pixel(500)
        .with_recursion_depth(50)
        .with_vertical_field_of_view(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .with_up(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.6)
        .with_focus_dist(10.0)
        .build();

    camera.render_to_io(&world, output)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let output = args.output.unwrap_or_else(|| PathBuf::from("/dev/stdout"));
    let file = std::fs::OpenOptions::new()
        .write(true)
        .read(false)
        .truncate(true)
        .create(true)
        .open(output.as_path())?;
    let mut writer = BufWriter::new(file);

    match args.scene {
        Scene::Spheres => spheres(&mut writer),
        Scene::BookCover => book_cover(&mut writer),
    }
}
