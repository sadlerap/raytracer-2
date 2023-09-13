use std::{ops::Range, sync::Arc};

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod sphere;
pub use sphere::Sphere;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn crate::material::Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Range<f32>) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Range<f32>) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.end;
        let mut rec = None;
        for object in self.objects.iter() {
            if let Some(record) = object.hit(r, &(ray_t.start..closest_so_far)) {
                closest_so_far = record.t;
                rec = Some(record);
            }
        }

        rec
    }
}
