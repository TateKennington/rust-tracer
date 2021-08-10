use super::Material;
use crate::geometry::{
    hittable::HitResult,
    ray::Ray,
    vector3::{Color, Vector3},
};
use rand::prelude::*;

pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Color, Ray)> {
        let attentuation = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ir = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let direction = ray.direction.unit();
        let cos_theta = Vector3::dot(&(-1.0f64 * &direction), &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let mut rng = thread_rng();

        let scatter_direction = if sin_theta * ir > 1.0 || reflectance(cos_theta, ir) > rng.gen() {
            direction.reflect(&hit.normal)
        } else {
            Vector3::refract(&direction, &hit.normal, ir)
        };

        let scatter_ray = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        return Some((attentuation, scatter_ray));
    }
}

fn reflectance(cos: f64, ir: f64) -> f64 {
    let mut r0 = (1.0 - ir) / (1.0 + ir);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
}
