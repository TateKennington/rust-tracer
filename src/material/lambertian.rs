use super::Material;
use crate::geometry::{
    hittable::HitResult,
    ray::Ray,
    vector3::{Color, Vector3},
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + Vector3::random_vec_sphere();
        if Vector3::near_zero(&scatter_direction) {
            scatter_direction = hit.normal;
        }

        let scatter_ray = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        return Some((self.albedo, scatter_ray));
    }
}
