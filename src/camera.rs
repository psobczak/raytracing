use crate::{ray::Ray, vec3::Vec3, AspectRatio};

#[derive(Debug)]
pub struct Camera {
    pub viewport: Viewport,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(viewport: Viewport) -> Self {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport.width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport.height, 0.0);
        let lower_left_corner = origin
            - (horizontal / 2.0)
            - (vertical / 2.0)
            - Vec3::new(0.0, 0.0, viewport.focal_length);
        Self {
            viewport,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    focal_length: f32,
    width: f32,
    height: f32,
}

impl Viewport {
    pub fn new(aspect_ratio: AspectRatio, height: f32, focal_length: f32) -> Self {
        Self {
            focal_length,
            height,
            width: aspect_ratio.as_f32() * height,
        }
    }
}
