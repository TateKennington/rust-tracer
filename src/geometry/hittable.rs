use super::ray::Ray;
use super::vector3::Vector3;
use crate::material::MaterialKind;

pub struct HitResult<'a> {
    pub point: Vector3,
    pub t: f64,
    pub normal: Vector3,
    pub front_face: bool,
    pub material: Option<&'a MaterialKind>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitResult>;
}
