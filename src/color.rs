use std::{fmt::Display, ops::Add, ops::Mul};

use crate::vec3::Vec3;

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

    pub fn lerp(start: Color, end: Color, t: f32) -> Color {
        (1.0 - t) * start + (t * end)
    }

    pub fn from_vec3(vec: Vec3) -> Self {
        Self {
            r: vec.x(),
            g: vec.y(),
            b: vec.z(),
        }
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

macro_rules! impl_single_type_operations {
    ($type:ty) => {
        impl Add<$type> for $type {
            type Output = Color;

            fn add(self, rhs: $type) -> Self::Output {
                Color::new(self.r + rhs.r, self.g + rhs.r, self.b + rhs.b)
            }
        }

        impl Mul<$type> for f32 {
            type Output = Color;

            fn mul(self, rhs: $type) -> Self::Output {
                Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
            }
        }
    };
}

impl_single_type_operations!(&Color);
impl_single_type_operations!(Color);
