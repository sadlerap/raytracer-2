use std::{
    f32::consts::PI,
    num::{NonZeroU32, NonZeroU64},
};

use indicatif::ProgressStyle;
use rayon::prelude::*;

use crate::{
    geometry::Hittable,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

/// A camera to render a scene with.
#[derive(Debug)]
pub struct Camera {
    samples_per_pixel: NonZeroU32,
    image_width: NonZeroU64,
    image_height: NonZeroU64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: NonZeroU32,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    /// Renders a PPM image to `output`.
    pub fn render_to_io<Output, World>(
        &self,
        world: &World,
        output: &mut Output,
    ) -> std::io::Result<()>
    where
        Output: std::io::Write,
        World: Hittable + std::marker::Sync,
    {
        write!(
            output,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        let progress_bar =
            indicatif::ProgressBar::new(u64::from(self.image_width) * u64::from(self.image_height))
                .with_message("Pixels written")
                .with_style(
                    ProgressStyle::with_template(
                        "[{elapsed_precise:8.green}] {bar:40.cyan} [ETA: {eta_precise:8.magenta}] {msg}: {percent:>3}%",
                    )
                    .unwrap(),
                );

        let progress_bar_ref = &progress_bar;

        let num_pixels = (u64::from(self.image_width) * u64::from(self.image_height)) as usize;
        let mut buffer = vec![Color::default(); num_pixels];

        buffer
            .par_iter_mut()
            .enumerate()
            .map(|(index, dest)| {
                let i = index as u64 % u64::from(self.image_width);
                let j = index as u64 / u64::from(self.image_width);
                (i, j, dest)
            })
            .for_each(move |(i, j, dest)| {
                let color: Color = (0..u32::from(self.samples_per_pixel))
                    .map(|_| self.get_ray(i, j))
                    .map(|ray| self.ray_color(&ray, self.max_depth.into(), world))
                    .sum();
                *dest = color;
                progress_bar_ref.inc(1);
            });

        for color in buffer {
            color.write_ppm(output, self.samples_per_pixel)?;
        }

        progress_bar.finish_and_clear();

        eprintln!("Done!");
        Ok(())
    }

    /// Samples a ray for the pixel at (i, j).
    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px: f32 = rand::random::<f32>() - 0.5;
        let py: f32 = rand::random::<f32>() - 0.5;

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn ray_color<World: Hittable>(&self, ray: &Ray, depth: u32, world: &World) -> Color {
        // if we've exceeded max depth, don't gather any more light.
        if depth == 0 {
            return Color::default();
        }

        if let Some(record) = world.hit(ray, &(0.001..f32::INFINITY)) {
            if let Some((scattered, attenuation)) = record.material.scatter(ray, &record) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            } else {
                return Color::default();
            }
        }
        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let color_1 = Color::new(1.0, 1.0, 1.0);
        let color_2 = Color::new(0.5, 0.7, 1.0);

        (1.0 - a) * color_1 + a * color_2
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disc();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}

#[derive(Debug, Default)]
pub struct CameraBuilder {
    pub aspect_ratio: Option<f32>,
    pub samples_per_pixel: Option<NonZeroU32>,
    pub image_width: Option<NonZeroU64>,
    pub max_depth: Option<NonZeroU32>,
    pub vfov: Option<f32>,
    pub look_from: Option<Point3>,
    pub look_at: Option<Point3>,
    pub up: Option<Vec3>,
    pub defocus_angle: Option<f32>,
    pub focus_dist: Option<f32>,
}

impl From<CameraBuilder> for Camera {
    fn from(val: CameraBuilder) -> Self {
        let aspect_ratio = val.aspect_ratio.unwrap_or(1.0);
        let samples_per_pixel = val
            .samples_per_pixel
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(10) });

        let max_depth = val
            .max_depth
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(10) });

        let image_width = val
            .image_width
            // Safety: new_unchecked requires the argument to be non-zero, which 100 satisfies.
            .unwrap_or_else(|| unsafe { NonZeroU64::new_unchecked(100) });

        let image_height =
            NonZeroU64::new(((u64::from(image_width) as f32 / aspect_ratio) as u64).max(1))
                .expect("Image width is zero");

        let vfov = val.vfov.unwrap_or(90.0);
        let look_from = val.look_from.unwrap_or_else(|| Point3::new(0.0, 0.0, -1.0));
        let look_at = val.look_at.unwrap_or_else(|| Point3::new(0.0, 0.0, 0.0));
        let up = val.up.unwrap_or_else(|| Vec3::new(0.0, 1.0, 0.0));

        let defocus_angle = val.defocus_angle.unwrap_or(0.0);
        let focus_dist = val.focus_dist.unwrap_or(10.0);

        let center = look_from;

        // determine viewport dimensions
        let theta = vfov * (PI / 180.0);
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        // calculate the u,v,w unit basis vectors for the camera coordinate frame
        let w = (look_from - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / u64::from(image_width) as f32;
        let pixel_delta_v = viewport_v / u64::from(image_height) as f32;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * ((0.5 * defocus_angle).to_radians()).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            samples_per_pixel,
            max_depth,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

impl CameraBuilder {
    /// Sets the aspect ratio for the camera.
    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }

    /// Sets the image width for the camera.  If the passed image width is `0`, then the camera
    /// uses the default of 100 pixels.
    pub fn with_image_width(mut self, image_width: u64) -> Self {
        self.image_width = NonZeroU64::new(image_width);
        self
    }

    /// Sets the sample depth for the camera.  If the passed sample depth is `0`, then the camera
    /// uses its default of `10` samples per pixel.
    pub fn with_samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = NonZeroU32::new(samples_per_pixel);
        self
    }

    /// Sets the recursion depth for ray reflections.  Defaults to 10.
    pub fn with_recursion_depth(mut self, depth: u32) -> Self {
        self.max_depth = NonZeroU32::new(depth);
        self
    }

    pub fn with_vertical_field_of_view(mut self, fov: f32) -> Self {
        self.vfov = Some(fov);
        self
    }

    pub fn look_from(mut self, from: Point3) -> Self {
        self.look_from = Some(from);
        self
    }

    pub fn look_at(mut self, at: Point3) -> Self {
        self.look_at = Some(at);
        self
    }

    pub fn with_up(mut self, up: Vec3) -> Self {
        self.up = Some(up);
        self
    }

    pub fn with_defocus_angle(mut self, angle: f32) -> Self {
        self.defocus_angle = Some(angle);
        self
    }

    pub fn with_focus_dist(mut self, dist: f32) -> Self {
        self.focus_dist = Some(dist);
        self
    }

    pub fn build(self) -> Camera {
        self.into()
    }
}
