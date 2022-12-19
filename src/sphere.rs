use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<impl Material + 'static>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_records: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot_product(&oc, ray.direction());
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !(t_min..=t_max).contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !(t_min..=t_max).contains(&root) {
                return false;
            }
        }

        hit_records.t = root;
        hit_records.point = ray.at(hit_records.t);
        let outward_normal = (hit_records.point - self.center) / self.radius;
        hit_records.set_face_normal(ray, &outward_normal);
        hit_records.material = Some(Rc::clone(&self.material));

        true
    }
}
