pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::geometry::{hittable::HitResult, ray::Ray, vector3::Color};
use dielectric::Dielectric;
use lambertian::Lambertian;
use metal::Metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Color, Ray)>;
}

pub enum MaterialKind {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for MaterialKind {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Color, Ray)> {
        match self {
            MaterialKind::Lambertian(lambertian) => lambertian.scatter(ray, hit),
            MaterialKind::Metal(metal) => metal.scatter(ray, hit),
            MaterialKind::Dielectric(dielectric) => dielectric.scatter(ray, hit),
        }
    }
}
