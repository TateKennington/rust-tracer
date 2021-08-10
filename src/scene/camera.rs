use crate::geometry::ray::Ray;
use crate::geometry::vector3::Vector3;

pub struct Camera {
    origin: Vector3,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
}

impl Camera {
    pub fn new(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Camera {
        Camera {
            origin: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            viewport_height,
            viewport_width,
            focal_length,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let horizontal = &Vector3 {
            x: self.viewport_width,
            y: 0.0,
            z: 0.0,
        };

        let vertical = &Vector3 {
            x: 0.0,
            y: -self.viewport_height,
            z: 0.0,
        };

        let viewport_origin = Vector3 {
            x: -self.viewport_width / 2.0,
            y: self.viewport_height / 2.0,
            z: -self.focal_length,
        };
        Ray {
            origin: self.origin,
            direction: u * horizontal + v * vertical + viewport_origin + self.origin * -1.0,
        }
    }
}
