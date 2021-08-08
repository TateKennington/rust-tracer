use super::hittable::{HitResult, Hittable};
use super::ray::Ray;
use super::vector3::Vector3;

pub struct Sphere {
    pub origin: Vector3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitResult> {
        let sphere_to_ray = &ray.origin - &self.origin;
        let half_b = Vector3::dot(&ray.direction, &sphere_to_ray);
        let direction_sq = ray.direction.len_sq();
        let determinant =
            half_b * half_b - direction_sq * (sphere_to_ray.len_sq() - self.radius * self.radius);

        if determinant < 0.0 {
            return None;
        }

        let mut root = (-half_b - determinant.sqrt()) / direction_sq;
        if root < min_t || root > max_t {
            root = (-half_b + determinant.sqrt()) / direction_sq;
            if root < min_t || root > max_t {
                return None;
            }
        }

        let point = ray.at(root);
        let mut normal = (&point - &self.origin).unit();
        let front_face = Vector3::dot(&ray.direction, &normal) <= 0.0;

        if !front_face {
            normal *= -1.0;
        }

        return Some(HitResult {
            point,
            normal,
            front_face,
            t: root,
            material: None,
        });
    }
}
