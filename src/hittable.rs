use std::rc::Rc;

use crate::{material::Material, ray::Ray, vec3::Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_records: &mut HitRecord) -> bool;
}

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot_product(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        }
    }
}

#[derive(Default)]
pub struct HittableList(Vec<Rc<dyn Hittable>>);

impl HittableList {
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.0.push(Rc::new(object))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_records: &mut HitRecord) -> bool {
        self.0
            .iter()
            .any(|object| object.hit(ray, t_min, t_max, hit_records))
    }
}
