use std::{
    marker::{Send, Sync},
    ops::Range,
};

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod sphere;
pub use sphere::Sphere;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal).is_sign_negative();
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Range<f32>) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList<'a> {
    objects: Vec<&'a (dyn Hittable + Sync + Send)>,
}

impl<'a> HittableList<'a> {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: &'a (dyn Hittable + Sync + Send)) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList<'_> {
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
