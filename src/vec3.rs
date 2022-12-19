use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn unit_vector(vector: &Vec3) -> Vec3 {
        vector.length() / vector
    }

    pub fn dot_product(first: &Vec3, other: &Vec3) -> f32 {
        (first.x * other.x) + (first.y * other.y) + (first.z * other.z)
    }

    pub fn random(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let point = Vec3::random(-1.0, 1.0);
            if point.length_squared() >= 1.0 {
                continue;
            }

            return point;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::unit_vector(&Self::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f32::abs(self.x) < s) && (f32::abs(self.y) < s) && (f32::abs(self.z) < s)
    }

    pub fn reflect(first: &Vec3, other: &Vec3) -> Vec3 {
        first - (2.0 * Vec3::dot(first, other) * other)
    }
}

macro_rules! impl_double_type_operations {
    ($first_type:ty, $second_type:ty) => {
        impl Add<$first_type> for $second_type {
            type Output = Vec3;

            fn add(self, rhs: $first_type) -> Self::Output {
                Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
            }
        }

        impl Sub<$first_type> for $second_type {
            type Output = Vec3;

            fn sub(self, rhs: $first_type) -> Self::Output {
                Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
            }
        }
    };
}

macro_rules! impl_single_type_operations {
    ($type:ty) => {
        impl Mul<$type> for f32 {
            type Output = Vec3;

            fn mul(self, rhs: $type) -> Self::Output {
                Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
            }
        }

        impl Mul<f32> for $type {
            type Output = Vec3;

            fn mul(self, rhs: f32) -> Self::Output {
                Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
            }
        }

        impl Neg for $type {
            type Output = Vec3;

            fn neg(self) -> Self::Output {
                Vec3::new(-self.x, -self.y, -self.z)
            }
        }

        impl Div<$type> for f32 {
            type Output = Vec3;

            fn div(self, rhs: $type) -> Self::Output {
                (1.0 / self) * rhs
            }
        }

        impl Div<f32> for $type {
            type Output = Vec3;

            fn div(self, rhs: f32) -> Self::Output {
                (1.0 / rhs) * self
            }
        }
    };
}

impl_double_type_operations!(Vec3, Vec3);
impl_double_type_operations!(&Vec3, &Vec3);
impl_double_type_operations!(Vec3, &Vec3);
impl_double_type_operations!(&Vec3, Vec3);

impl_single_type_operations!(Vec3);
impl_single_type_operations!(&Vec3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding() {
        let first = Vec3::new(-10.0, 0.0, 25.0);
        let other = Vec3::new(-10.0, 0.0, 25.0);
        let expected = Vec3::new(-20.0, 0.0, 50.0);
        assert_eq!(first + other, expected)
    }

    #[test]
    fn substracting() {
        let first = Vec3::new(-10.0, 0.0, 25.0);
        let other = Vec3::new(-10.0, 0.0, 25.0);
        let expected = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(first - other, expected)
    }

    #[test]
    fn multiplying() {
        let first = Vec3::new(-10.0, 0.0, 25.0);
        let expected = Vec3::new(-25.0, 0.0, 62.5);
        assert_eq!(first * 2.5, expected)
    }

    #[test]
    fn dot_product() {
        let first = Vec3::new(1.0, 10.0, -25.0);
        let other = Vec3::new(12.1, -52.2, 0.0);
        assert_eq!(first.dot(&other), -509.9);
        assert_eq!(Vec3::dot_product(&first, &other), -509.9);
    }

    #[test]
    fn division() {
        let first = Vec3::new(1.0, 10.0, -20.0);
        assert_eq!(first / 2.0, Vec3::new(0.5, 5.0, -10.0))
    }
}
