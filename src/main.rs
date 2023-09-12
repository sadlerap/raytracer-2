use std::{
    io::{BufWriter, Result},
    sync::Arc,
};

use crate::{
    geometry::{HittableList, Sphere},
    vec3::Point3,
};

pub mod camera;
pub mod geometry;
pub mod ray;
pub mod vec3;

fn main() -> Result<()> {
    let stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(stdout);

    // World

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let mut camera = camera::CameraBuilder::default()
        .with_image_width(1920)
        .with_aspect_ratio(16.0 / 9.0)
        .build();

    // Render

    camera.render_to_io(&mut world, &mut writer)?;

    Ok(())
}
