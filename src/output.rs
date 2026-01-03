use crate::{
    math::{Color, Interval, Ray},
    scene::Scene,
};

/// RGBA pixel in the format expected by canvas ImageData (4 bytes per pixel).
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color> for Pixel {
    fn from(color: Color) -> Self {
        let intensity = Interval::new(0.0, 0.999);
        Self {
            r: (256.0 * intensity.clamp(color.x)) as u8,
            g: (256.0 * intensity.clamp(color.y)) as u8,
            b: (256.0 * intensity.clamp(color.z)) as u8,
            a: 255,
        }
    }
}

#[repr(C)]
pub struct TargetOutput {
    data: Box<[Pixel]>,
    pub scene: Scene,
    pub width: u32,
    pub height: u32,
}

impl TargetOutput {
    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            data: vec![Pixel::default(); (width * height) as usize].into_boxed_slice(),
            scene: Scene::new(width, height),
            width,
            height,
        }
    }

    /// Sets the pixel at (x, y) to the given color.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let idx = (y * self.width + x) as usize;
        self.data[idx] = color.into();
    }

    /// Renders the scene into the target output's pixel buffer.
    pub fn render(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let pixel_center = self.scene.camera.pixel00_loc()
                    + (i as f32 * self.scene.camera.pixel_delta_x())
                    + (j as f32 * self.scene.camera.pixel_delta_y());

                let ray_direction = pixel_center - self.scene.camera.origin;

                let ray = Ray::new(self.scene.camera.origin, ray_direction);
                let color = self.scene.camera.ray_color(&ray, &self.scene.world);
                self.set_pixel(i, j, color);
            }
        }
    }
}

impl Drop for TargetOutput {
    fn drop(&mut self) {
        panic!("TargetOutput should never be dropped");
    }
}
