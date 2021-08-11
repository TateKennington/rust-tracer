use crate::geometry::ray::Ray;
use crate::geometry::vector3::Vector3;

pub struct Camera {
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    forward: Vector3,
    side: Vector3,
    up: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Vector3,
        look_at: Vector3,
        view_up: Vector3,
        aspect_ratio: f64,
        fov: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let forward = (origin - look_at).unit();
        let side = view_up.cross(&forward).unit();
        let up = forward.cross(&side);

        let horizontal = side * viewport_width * focus_dist;
        let vertical = up * -viewport_height * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            forward: forward * focus_dist,
            lens_radius: aperture / 2.0,
            side,
            up,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vector3::random_vec_disk() * self.lens_radius;
        let offset = self.side * rd.x + self.up * rd.y;
        let viewport_origin =
            self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.forward;

        Ray {
            origin: self.origin + offset,
            direction: u * self.horizontal + v * self.vertical + viewport_origin
                - self.origin
                - offset,
        }
    }
}
