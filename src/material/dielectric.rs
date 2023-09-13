use crate::{geometry::HitRecord, ray::Ray, vec3::Color};

use super::Material;

#[derive(Debug)]
pub struct Dielectric {
    ior: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            self.ior.recip()
        } else {
            self.ior
        };

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let use_schlick = reflectance(cos_theta, refraction_ratio) > rand::random();
        let direction = if cannot_refract || use_schlick {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction);

        Some((scattered, attenuation))
    }
}

impl Dielectric {
    pub fn new(ior: f32) -> Self {
        Self { ior }
    }
}

fn reflectance(cosine: f32, ref_ior: f32) -> f32 {
    // Schlick's approximation
    let r0 = (1.0 - ref_ior) / (1.0 + ref_ior);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
