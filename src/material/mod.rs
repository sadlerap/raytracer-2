use crate::{ray::Ray, geometry::HitRecord, vec3::Color};

mod lambertian;
mod metal;

pub use lambertian::*;
pub use metal::*;

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}
