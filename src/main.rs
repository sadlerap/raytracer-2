use indicatif::ProgressStyle;
use std::{io::{BufWriter, Result, Write}, sync::Arc};

use crate::{vec3::{Point3, Vec3}, ray::Ray, geometry::{HittableList, Sphere}};

pub mod ray;
pub mod vec3;
pub mod geometry;

fn main() -> Result<()> {
    let stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(stdout);

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = ((image_width as f32 / aspect_ratio) as u64).max(1);

    // World

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32) / (image_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate teh horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    write!(&mut writer, "P3\n{image_width} {image_height}\n255\n")?;

    let progress_bar = indicatif::ProgressBar::new(image_width * image_height)
        .with_message("Pixels written")
        .with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan} {msg}: {percent:>3}%")
                .unwrap(),
        );

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let color = r.color(&world);
            color.write_ppm(&mut writer)?;
        }
        progress_bar.inc(image_width);
    }

    progress_bar.finish_and_clear();

    eprintln!("Done!");

    Ok(())
}
