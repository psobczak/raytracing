use crate::{color::Color, hit_sphere, vec3::Vec3};

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f32) -> Vec3 {
        &self.origin + (t * &self.direction)
    }

    pub fn ray_color(&self, start_color: Color, end_color: Color) -> Color {
        let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, self);

        if t > 0.0 {
            let n = Vec3::unit_vector(&(&self.at(t) - Vec3::new(0.0, 0.0, -1.0)));
            return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        }

        let unit_direction = Vec3::unit_vector(self.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::lerp(start_color, end_color, t)
    }
}
