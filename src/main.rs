use std::{
    io::{BufWriter, Result},
    sync::Arc,
};

use material::Dielectric;
use vec3::Color;

use crate::{
    geometry::{HittableList, Sphere},
    vec3::Point3, material::{Lambertian, Metal},
};

pub mod camera;
pub mod geometry;
mod material;
pub mod ray;
mod util;
pub mod vec3;

fn main() -> Result<()> {
    let stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(stdout);

    // Materials

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera

    let camera = camera::CameraBuilder::default()
        .with_image_width(1280)
        .with_aspect_ratio(16.0 / 9.0)
        .with_samples_per_pixel(500)
        .with_recursion_depth(10)
        .build();

    // Render

    camera.render_to_io(&world, &mut writer)?;

    Ok(())
}
