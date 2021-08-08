use rand::prelude::*;

const e: f64 = 1e-8;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    fn dot(vec: &Vector3, other: &Vector3) -> f64 {
        vec.x * other.x + vec.y * other.y + vec.z * other.z
    }

    fn reflect(&self, other: &Vector3) -> Vector3 {
        let proj = Vector3::dot(self, other);
        self - 2.0 * proj * other
    }

    fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit(&self) -> Vector3 {
        self / self.len()
    }

    pub fn lerp(vec: &Vector3, other: &Vector3, t: f64) -> Vector3 {
        (1.0 - t) * vec + t * other
    }

    fn near_zero(vec: Vector3) -> bool {
        vec.x.abs() < e && vec.y.abs() < e && vec.z.abs() < e
    }

    fn random_vec(min: f64, max: f64) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3 {
            x: min + rng.gen::<f64>() * (max - min),
            y: min + rng.gen::<f64>() * (max - min),
            z: min + rng.gen::<f64>() * (max - min),
        }
    }

    fn random_vec_sphere() -> Vector3 {
        loop {
            let vec = Vector3::random_vec(-1.0, 1.0);
            if vec.len() <= 1.0 {
                return vec.unit();
            }
        }
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Vector3 {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl std::ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
