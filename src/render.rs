use crate::Image;

pub trait Renderer {
    fn render(&self) {}
}

#[derive(Debug)]
pub struct Console<'a> {
    image: &'a Image,
}

impl<'a> Console<'a> {
    pub fn new(image: &'a Image) -> Self {
        Self { image }
    }
}

impl<'a> Renderer for Console<'a> {
    fn render(&self) {
        println!("P3");
        println!("{} {}", self.image.width, self.image.height);
        println!("255");

        for i in 0..self.image.height {
            for j in 0..self.image.width {
                let r = i as f32 / (self.image.width - 1) as f32;
                let g = j as f32 / (self.image.height - 1) as f32;
                let b = 0.25_f32;

                let ir = (255.99 * r) as usize;
                let ig = (255.99 * g) as usize;
                let ib = (255.99 * b) as usize;

                println!("{ir} {ig} {ib}")
            }
        }
    }
}
