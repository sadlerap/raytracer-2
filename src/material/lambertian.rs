use crate::{
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

/// Lambertian (diffuse) materials.
#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &crate::geometry::HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_on_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
