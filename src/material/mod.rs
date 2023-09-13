use crate::{geometry::HitRecord, ray::Ray, vec3::Color};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}
