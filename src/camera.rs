use std::num::NonZeroU64;

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
    pub image_width: NonZeroU64,
    image_height: NonZeroU64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render_to_io<Output: std::io::Write, World: Hittable>(
        &mut self,
        world: &World,
        writer: &mut Output,
    ) -> std::io::Result<()> {
        write!(
            writer,
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
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let color = self.ray_color(&r, world);
                color.write_ppm(writer)?;
            }
            progress_bar.inc(self.image_width.into());
        }

        progress_bar.finish_and_clear();

        eprintln!("Done!");
        Ok(())
    }

    fn ray_color<World: Hittable>(&self, ray: &Ray, world: &World) -> Color {
        if let Some(record) = world.hit(ray, &(0.0..f32::INFINITY)) {
            return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
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
    pub image_width: Option<NonZeroU64>,
}

impl From<CameraBuilder> for Camera {
    fn from(val: CameraBuilder) -> Self {
        let aspect_ratio = val.aspect_ratio.unwrap_or(1.0);
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

    /// Sets the image width for the camera.  If the passed image width is zero, then the camera
    /// uses its default.
    pub fn with_image_width(mut self, image_width: u64) -> Self {
        self.image_width = NonZeroU64::new(image_width);
        self
    }

    pub fn build(self) -> Camera {
        self.into()
    }
}
