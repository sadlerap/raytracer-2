use crate::{vec3::{Color, Vec3}, ray::Ray};

use super::Material;

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self { Self { albedo, fuzz } }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &crate::geometry::HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().normalize().reflect(hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected + self.fuzz * Vec3::random_on_unit_sphere());
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
