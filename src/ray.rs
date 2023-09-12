use crate::{vec3::{Point3, Vec3, Color}, geometry::Hittable};

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        return self.origin + t * self.direction
    }

    pub fn color<T: Hittable>(&self, world: &T) -> Color {
        if let Some(record) = world.hit(self, &(0.0..f32::INFINITY)) {
            return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0))
        }
        let unit_direction = self.direction().normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let color_1 = Color::new(1.0, 1.0, 1.0);
        let color_2 = Color::new(0.5, 0.7, 1.0);

        (1.0 - a) * color_1 + a * color_2
    }
}
