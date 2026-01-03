use crate::{
    hittable::{HitRecord, Hittable},
    math::{Interval, Ray, Vec3},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-b - sqrt_d) / (2.0 * a);
        if !ray_t.surrounds(root) {
            root = (-b + sqrt_d) / (2.0 * a);
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);

        let outward_normal = (point - self.center) / self.radius;
        let (normal, front_face) = self.face_normal(ray, outward_normal);

        Some(HitRecord {
            point,
            normal,
            t: root,
            front_face,
        })
    }
}
