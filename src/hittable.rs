use std::{fmt::Debug, rc::Rc};

use crate::{ray::Ray, vec3::Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_records: &mut HitRecord) -> bool;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
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
    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.0.push(Rc::new(object))
    }

    pub fn get_list(self) -> impl IntoIterator<Item = Rc<dyn Hittable>> {
        self.0
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_records: &mut HitRecord) -> bool {
        self.0
            .iter()
            .any(|object| object.hit(ray, t_min, t_max, hit_records))
    }
}

impl Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HittableList").field(&self.0.len()).finish()
    }
}
