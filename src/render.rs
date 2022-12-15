use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::ray::Ray;
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

        for j in (0..self.image.height).into_iter().rev() {
            for i in 0..self.image.width {
                let u = i as f32 / (self.image.width - 1) as f32;
                let v = j as f32 / (self.image.height - 1) as f32;

                let ray = Ray::new(
                    self.camera.origin,
                    self.camera.lower_left_corner
                        + (u * self.camera.horizontal)
                        + (v * self.camera.vertical)
                        - self.camera.origin,
                );

                let start_color = Color::new(1.0, 1.0, 1.0);
                let end_color = Color::new(0.5, 0.7, 1.0);

                let color = ray.ray_color(start_color, end_color, self.world);
                println!("{color}")
            }
        }
    }
}
