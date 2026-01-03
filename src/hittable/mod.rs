use crate::math::{Interval, Point3, Ray, Vec3};

pub mod sphere;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn face_normal(&self, ray: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (normal, front_face)
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                // println!("Hit at t={}", temp_rec.t);
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }
}
