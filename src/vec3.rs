use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
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

    pub fn dot(&self, other: Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

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
}
