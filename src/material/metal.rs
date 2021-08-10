use super::Material;
use crate::geometry::{
    hittable::HitResult,
    ray::Ray,
    vector3::{Color, Vector3},
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Color, Ray)> {
        let mut scatter_direction =
            ray.direction.reflect(&hit.normal) + self.fuzz * &Vector3::random_vec_sphere();

        let scatter_ray = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        if Vector3::dot(&scatter_ray.direction, &hit.normal) >= 0.0 {
            return Some((self.albedo, scatter_ray));
        }
        return None;
    }
}
