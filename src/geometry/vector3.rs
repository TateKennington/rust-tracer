use rand::prelude::*;
use std::ops;

const E: f64 = 1e-8;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn dot(vec: &Vector3, other: &Vector3) -> f64 {
        vec.x * other.x + vec.y * other.y + vec.z * other.z
    }

    pub fn reflect(&self, other: &Vector3) -> Vector3 {
        let proj = Vector3::dot(self, other);
        self - 2.0 * proj * other
    }

    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn refract(uv: &Vector3, n: &Vector3, ir: f64) -> Vector3 {
        let cos_theta = Vector3::dot(&-uv, n).min(1.0);
        let perp = (uv + cos_theta * n) * ir;
        let parallel = -n * (1.0 - perp.len_sq()).sqrt();
        perp + parallel
    }

    pub fn unit(&self) -> Vector3 {
        self / self.len()
    }

    pub fn lerp(vec: &Vector3, other: &Vector3, t: f64) -> Vector3 {
        (1.0 - t) * vec + t * other
    }

    pub fn near_zero(vec: &Vector3) -> bool {
        vec.x.abs() < E && vec.y.abs() < E && vec.z.abs() < E
    }

    fn random_vec(min: f64, max: f64) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3 {
            x: min + rng.gen::<f64>() * (max - min),
            y: min + rng.gen::<f64>() * (max - min),
            z: min + rng.gen::<f64>() * (max - min),
        }
    }

    pub fn random_vec_disk() -> Vector3 {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Vector3 {
                x: -1.0 + rng.gen::<f64>(),
                y: -1.0 + rng.gen::<f64>(),
                z: 0.0,
            };
            if vec.len() <= 1.0 {
                return vec;
            }
        }
    }

    pub fn random_vec_sphere() -> Vector3 {
        loop {
            let vec = Vector3::random_vec(-1.0, 1.0);
            if vec.len() <= 1.0 {
                return vec.unit();
            }
        }
    }

    pub fn sqrt(&mut self) {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self.z = self.z.sqrt();
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3{
    Vector3::new(a.x + b.x, a.y + b.y, a.z+b.z)
});

impl_op_ex!(-|a: &Vector3, b: &Vector3| -> Vector3 {
    Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

impl_op_ex!(+= |a: &mut Vector3, b: &Vector3|{
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(-= |a: &mut Vector3, b: &Vector3|{
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op!(*= |a: &mut Vector3, b: f64|{
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op!(/= |a: &mut Vector3, b: f64|{
    a.x /= b;
    a.y /= b;
    a.z /= b;
});

impl_op_ex!(-|a: &Vector3| -> Vector3 { Vector3::new(-a.x, -a.y, -a.z) });

impl_op_ex_commutative!(*|a: &Vector3, b: f64| -> Vector3 {
    Vector3::new(a.x * b, a.y * b, a.z * b)
});

impl_op_ex_commutative!(/|a: &Vector3, b: f64| -> Vector3 {
    Vector3::new(a.x / b, a.y / b, a.z / b)
});

pub type Color = Vector3;
