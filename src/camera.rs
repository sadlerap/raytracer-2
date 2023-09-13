use std::num::{NonZeroU32, NonZeroU64};

use indicatif::ProgressStyle;

use crate::{
    geometry::Hittable,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

/// A camera to render a scene with.
#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub samples_per_pixel: NonZeroU32,
    pub image_width: NonZeroU64,
    image_height: NonZeroU64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: NonZeroU32,
}

impl Camera {
    /// Renders a PPM image to `output`.
    pub fn render_to_io<Output: std::io::Write, World: Hittable>(
        &self,
        world: &World,
        output: &mut Output,
    ) -> std::io::Result<()> {
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
                        "[{elapsed_precise}] {bar:40.cyan} {msg}: {percent:>3}%",
                    )
                    .unwrap(),
                );

        for j in 0..self.image_height.into() {
            for i in 0..self.image_width.into() {
                let color: Color = (0..u32::from(self.samples_per_pixel))
                    .map(|_| self.get_ray(i, j))
                    .map(|ray| self.ray_color(&ray, self.max_depth.into(), world))
                    .sum();
                color.write_ppm(output, self.samples_per_pixel)?;
            }
            progress_bar.inc(self.image_width.into());
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

        Ray::new(self.center, pixel_sample - self.center)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px: f32 = -0.5 + rand::random::<f32>();
        let py: f32 = -0.5 + rand::random::<f32>();

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
}

#[derive(Debug, Default)]
pub struct CameraBuilder {
    pub aspect_ratio: Option<f32>,
    pub samples_per_pixel: Option<NonZeroU32>,
    pub image_width: Option<NonZeroU64>,
    pub max_depth: Option<NonZeroU32>,
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

        let center = Point3::default();
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / u64::from(image_width) as f32;
        let pixel_delta_v = viewport_v / u64::from(image_height) as f32;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u * 0.5) - (viewport_v * 0.5);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
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

    pub fn build(self) -> Camera {
        self.into()
    }
}
