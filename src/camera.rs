use crate::{vec3::Vec3, AspectRatio};

#[derive(Debug)]
pub struct Camera {
    pub viewport: Viewport,
    pub focal_length: f32,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertival: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(viewport: Viewport, focal_length: f32) -> Self {
        let origin = Vec3::default();
        let horizontal = Vec3::new(viewport.height, 0.0, 0.0);
        Self {
            viewport,
            focal_length,
            origin,
            horizontal,
            vertival: Vec3::new(0.0, viewport.height, 0.0),
            lower_left_corner: origin - (horizontal / 2.0) - Vec3::new(0.0, 0.0, focal_length),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    width: f32,
    height: f32,
}

impl Viewport {
    pub fn new(aspect_ratio: AspectRatio, height: f32) -> Self {
        Self {
            height,
            width: aspect_ratio.as_f32(),
        }
    }
}
