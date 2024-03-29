use crate::{
    geometry::{HitRecord, Hittable},
    material::Material,
    vec3::Point3,
};

#[derive(Debug)]
pub struct Sphere<'a> {
    center: Point3,
    radius: f32,
    radius_recip: f32,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f32, material: &'a dyn Material) -> Self {
        Self {
            center,
            radius,
            radius_recip: radius.recip(),
            material,
        }
    }
}

impl Hittable for Sphere<'_> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &std::ops::Range<f32>) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().len_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let a_recip = a.recip();
        let mut root = (-half_b - sqrtd) * a_recip;
        if !ray_t.contains(&root) {
            root = (-half_b + sqrtd) * a_recip;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let point = r.at(root);
        let normal = (point - self.center) * self.radius_recip;
        let mut hit_record = HitRecord {
            point,
            normal,
            t: root,
            front_face: false,
            material: self.material,
        };
        let outward_normal = (point - self.center) * self.radius_recip;
        hit_record.set_face_normal(r, outward_normal);
        Some(hit_record)
    }
}
