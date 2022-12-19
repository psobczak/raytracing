use rand::Rng;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::Image;

pub trait Renderer {
    fn render(&self) {}
}

#[derive(Debug)]
pub struct Console<'a> {
    image: &'a Image,
    camera: &'a Camera,
    world: &'a HittableList,
}

impl<'a> Console<'a> {
    pub fn new(image: &'a Image, camera: &'a Camera, world: &'a HittableList) -> Self {
        Self {
            image,
            camera,
            world,
        }
    }
}

impl<'a> Renderer for Console<'a> {
    fn render(&self) {
        println!("P3");
        println!("{} {}", self.image.width, self.image.height);
        println!("255");

        let mut rng = rand::thread_rng();

        for j in (0..self.image.height).into_iter().rev() {
            for i in 0..self.image.width {
                (0..self.image.samples_per_pixel)
                    .map(|_| {
                        (
                            (i as f32 + rng.gen::<f32>()) / (self.image.width - 1) as f32, // u
                            (j as f32 + rng.gen::<f32>()) / (self.image.height - 1) as f32, // v
                        )
                    })
                    .map(|(u, v)| self.camera.get_ray(u, v))
                    .fold(Color::default(), |acc, ray| {
                        acc + ray.color(self.world, self.image.max_depth)
                    })
                    .write(self.image.samples_per_pixel);
            }
        }
    }
}
