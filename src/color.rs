use std::{ops::Add, ops::Mul};

use crate::{clamp, vec3::Vec3};

#[derive(Debug, Default, Clone, Copy)]
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

    pub fn write(&self, samples_per_pixel: usize) {
        let scale = 1.0 / samples_per_pixel as f32;
        let mut r = self.r;
        let mut g = self.g;
        let mut b = self.b;

        r = f32::sqrt(scale * r);
        g = f32::sqrt(scale * g);
        b = f32::sqrt(scale * b);

        println!(
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as usize,
            (256.0 * clamp(g, 0.0, 0.999)) as usize,
            (256.0 * clamp(b, 0.0, 0.999)) as usize
        )
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * self.g, self.b * rhs.b)
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
