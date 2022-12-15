mod camera;
mod color;
mod hittable;
mod ray;
mod render;
mod sphere;
mod vec3;

use camera::{Camera, Viewport};
use hittable::HittableList;
use render::{Console, Renderer};
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let aspect_ratio: AspectRatio = (16.0, 9.0).into();

    let image = Image::new(1000, aspect_ratio);

    let mut world = HittableList::default();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let viewport = Viewport::new(aspect_ratio, 2.0);
    let camera = Camera::new(viewport, 1.0);

    let console_renderer = Console::new(&image, &camera, &world);
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
}

impl From<(f32, f32)> for AspectRatio {
    fn from(value: (f32, f32)) -> Self {
        Self(value.0, value.1)
    }
}
