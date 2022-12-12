use render::{Console, Renderer};

mod render;
mod vec3;
mod color;

fn main() {
    let image = Image::new(256, 256);
    let console_renderer = Console::new(&image);
    console_renderer.render();
}

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}
