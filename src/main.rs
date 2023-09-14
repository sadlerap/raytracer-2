use std::io::{BufWriter, Result};

use material::{Dielectric, Metal};
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

fn main() -> Result<()> {
    let stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(stdout);

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

    camera.render_to_io(&world, &mut writer)?;
    drop(world);

    Ok(())
}
