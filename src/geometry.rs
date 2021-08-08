pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vector3;

use hittable::{HitResult, Hittable};
use ray::Ray;
use sphere::Sphere;

pub enum Geometry {
    Sphere(Sphere),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitResult> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray, min_t, max_t),
        }
    }
}
