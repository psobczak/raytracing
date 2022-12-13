use std::fmt::Display;

#[derive(Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.r * 255.999) as usize,
            (self.g * 255.999) as usize,
            (self.b * 255.999) as usize
        )
    }
}
