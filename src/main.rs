mod camera;
mod color;
mod ray;
mod render;
mod sphere;
mod vec3;

use camera::{Camera, Viewport};
use ray::Ray;
use render::{Console, Renderer};
use vec3::Vec3;

fn main() {
    let aspect_ratio: AspectRatio = (16.0, 9.0).into();

    let image = Image::new(400, aspect_ratio);

    let viewport = Viewport::new(aspect_ratio, 2.0);
    let camera = Camera::new(viewport, 1.0);

    let console_renderer = Console::new(&image, &camera);
    console_renderer.render();
}

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
}

impl Image {
    fn new(width: usize, aspect_ratio: AspectRatio) -> Self {
        Self {
            width,
            height: (width as f32 / aspect_ratio.as_f32()) as usize,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AspectRatio(f32, f32);

impl AspectRatio {
    pub fn as_f32(&self) -> f32 {
        self.0 / self.1
    }

    pub fn as_usize(&self) -> usize {
        (self.0 / self.1) as usize
    }
}

impl From<(f32, f32)> for AspectRatio {
    fn from(value: (f32, f32)) -> Self {
        Self(value.0, value.1)
    }
}

pub fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(&oc) - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

