use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction,
        }
    }

    // Schlick's approximation for reflectance
    fn reflectance(cosine: f32, index_of_refraction: f32) -> f32 {
        let mut r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        r0 = f32::powi(r0, 2);
        r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let unit_direction = Vec3::unit_vector(ray.direction());

        let cos_theta = f32::min(Vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = Self::reflectance(cos_theta, refraction_ratio) > rng.gen();

        let direction = match (cannot_refract, reflectance) {
            (false, false) => Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio),
            _ => Vec3::reflect(&unit_direction, &hit_record.normal),
        };

        *scattered = Ray::new(hit_record.point, direction);
        true
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(ray.direction()), &hit_record.normal);
        *scattered = Ray::new(
            hit_record.point,
            reflected + (self.fuzz * Vec3::random_in_unit_sphere()),
        );
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), &hit_record.normal) > 0.0
    }
}
