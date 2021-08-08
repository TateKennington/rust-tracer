use crate::geometry::hittable::{HitResult, Hittable};
use crate::geometry::ray::Ray;
use crate::geometry::Geometry;
use crate::material::MaterialKind;

pub struct Scene {
    pub objects: Vec<Object>,
}

pub struct Object {
    geometry: Geometry,
    material: MaterialKind,
}

impl Scene {
    pub fn add(&mut self, geometry: Geometry, material: MaterialKind) {
        self.objects.push(Object { geometry, material });
    }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitResult> {
        if let Some(mut hit_result) = self.geometry.hit(ray, min_t, max_t) {
            hit_result.material = Some(&self.material);
            Some(hit_result)
        } else {
            None
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitResult> {
        let mut hit: Option<HitResult> = None;
        for object in self.objects.iter() {
            let best_t = if let Some(best_hit) = &hit {
                best_hit.t
            } else {
                max_t
            };
            if let Some(new_hit) = object.hit(ray, min_t, best_t) {
                hit = Some(new_hit);
            }
        }
        hit
    }
}
