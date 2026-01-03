use crate::{
    hittable::{Hittable, HittableList, sphere::Sphere},
    math::{Color, Interval, Point3, Ray, Vec3},
};

pub struct Scene {
    pub camera: Camera,
    pub world: HittableList,
}

impl Scene {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let mut world = HittableList::new();
        world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

        Self {
            camera: Camera::new(image_width, image_height),
            world,
        }
    }
}

pub struct Camera {
    pub origin: Point3,
    focal_length: f32,
    viewport_width: f32,
    viewport_height: f32,
    image_width: u32,
    image_height: u32,
}

impl Camera {
    /// Creates a new camera with the given image dimensions.
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let aspect_ratio = image_width as f32 / image_height as f32;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Self {
            origin: Point3::default(),
            focal_length,
            viewport_width,
            viewport_height,
            image_width,
            image_height,
        }
    }

    /// Returns the horizontal edge vector of the viewport (left to right).
    pub fn viewport_horizontal(&self) -> Point3 {
        Point3::new(self.viewport_width, 0.0, 0.0)
    }

    /// Returns the vertical edge vector of the viewport (top to bottom).
    pub fn viewport_vertical(&self) -> Point3 {
        Point3::new(0.0, -self.viewport_height, 0.0)
    }

    /// Returns a vector representing the delta per pixel in the x direction.
    pub fn pixel_delta_x(&self) -> Vec3 {
        self.viewport_horizontal() / self.image_width as f32
    }

    /// Returns a vector representing the delta per pixel in the y direction.
    pub fn pixel_delta_y(&self) -> Vec3 {
        self.viewport_vertical() / self.image_height as f32
    }

    /// Returns the upper-left corner of the viewport in world space.
    pub fn viewport_upper_left(&self) -> Point3 {
        self.origin
            - Point3::new(0.0, 0.0, self.focal_length)
            - self.viewport_horizontal() / 2.0
            - self.viewport_vertical() / 2.0
    }

    /// Returns the location of pixel (0, 0) center.
    pub fn pixel00_loc(&self) -> Point3 {
        self.viewport_upper_left() + 0.5 * (self.pixel_delta_x() + self.pixel_delta_y())
    }

    /// Computes the color seen along the given ray in the scene.
    pub fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        if let Some(hit_record) = world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
            return 0.5
                * Color::new(
                    hit_record.normal.x + 1.0,
                    hit_record.normal.y + 1.0,
                    hit_record.normal.z + 1.0,
                );
        }

        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
