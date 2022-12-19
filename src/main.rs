mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod render;
mod sphere;
mod vec3;

use std::rc::Rc;

use camera::{Camera, Viewport};
use color::Color;
use hittable::HittableList;
use material::{Lambertian, Metal};
use render::{Console, Renderer};
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let aspect_ratio: AspectRatio = (16.0, 9.0).into();

    let image = Image::new(1000, aspect_ratio, 100, 25);

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let mut world = HittableList::default();
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let viewport = Viewport::new(aspect_ratio, 2.0, 1.0);
    let camera = Camera::new(viewport);

    let console_renderer = Console::new(&image, &camera, &world);
    console_renderer.render();
}

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Image {
    fn new(
        width: usize,
        aspect_ratio: AspectRatio,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            width,
            height: (width as f32 / aspect_ratio.as_f32()) as usize,
            samples_per_pixel,
            max_depth,
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

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }

    x
}
