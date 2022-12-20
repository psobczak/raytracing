mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod render;
mod sphere;
mod utils;
mod vec3;

use std::rc::Rc;

use camera::{Camera, Viewport};
use color::Color;
use hittable::HittableList;
use material::Lambertian;
use render::{Console, Renderer};
use sphere::Sphere;
use utils::AspectRatio;
use vec3::Vec3;

fn main() {
    let aspect_ratio: AspectRatio = (16.0, 9.0).into();

    let image = Image::new(1000, aspect_ratio, 100, 25);

    let r = f32::cos(std::f32::consts::PI / 4.0);

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let mut world = HittableList::default();

    world.add(Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        Rc::clone(&material_left),
    ));

    world.add(Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        Rc::clone(&material_right),
    ));

    let viewport = Viewport::new(aspect_ratio, 1.0, 90.0);
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
