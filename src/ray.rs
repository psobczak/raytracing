use crate::{
    color::Color,
    hittable::{HitRecord, Hittable, HittableList},
    vec3::Vec3,
};

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
        self.origin + (t * self.direction)
    }

    pub fn color(&self, world: &HittableList, depth: usize) -> Color {
        let mut hit_record = HitRecord::default();

        if depth <= 0 {
            return Color::default();
        }

        if world.hit(self, 0.001, f32::INFINITY, &mut hit_record) {
            let target = hit_record.point + Vec3::random_in_hemisphere(&hit_record.normal);
            return 0.5
                * Ray::new(hit_record.point, target - hit_record.point).color(world, depth - 1);
        }

        let start_color = Color::new(1.0, 1.0, 1.0);
        let end_color = Color::new(0.5, 0.7, 1.0);

        let unit_direction = Vec3::unit_vector(self.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::lerp(start_color, end_color, t)
    }
}
