use super::Material;
use crate::geometry::{
    hittable::HitResult,
    ray::Ray,
    vector3::{Color, Vector3},
};

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Color, Ray)> {
        let mut scatter_direction = ray.direction.reflect(&hit.normal);

        let scatter_ray = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        return Some((self.albedo, scatter_ray));
    }
}
