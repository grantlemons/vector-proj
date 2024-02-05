pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl std::ops::Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<(f32, f32, f32)> for Vector {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl Vector {
    pub fn new<T: Into<f32>>(x: T, y: T, z: T) -> Self {
        Vector {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn rotate_x(mut self, angle: f32) {
        let (sin, cos) = f32::sin_cos(angle);

        self.x = self.x;
        self.y = (self.y * cos) - (self.z * sin);
        self.z = (self.y * sin) + (self.z * cos);
    }

    pub fn rotate_y(mut self, angle: f32) {
        let (sin, cos) = f32::sin_cos(angle);

        self.x = (self.x * cos) + (self.z * sin);
        self.y = self.y;
        self.z = (-self.x * sin) + (self.z * cos);
    }

    pub fn rotate_z(mut self, angle: f32) {
        let (sin, cos) = f32::sin_cos(angle);

        self.x = (self.x * cos) - (self.y * sin);
        self.y = (self.x * sin) + (self.y * cos);
        self.z = self.z;
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
    }

    pub fn unit(&self) -> Vector {
        self * (1_f32 / self.magnitude())
    }

    pub fn proj(lhs: &Self, rhs: &Self) -> Self {
        lhs * ((rhs.dot_product(lhs)) / (lhs.dot_product(lhs)))
    }

    pub fn dot_product(&self, rhs: &Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross_product(&self, rhs: &Self) -> Self {
        Vector {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    pub fn proj_onto(&self, rhs: &Self) -> Self {
        Self::proj(rhs, self)
    }

    pub fn angle(&self, rhs: &Self) -> f32 {
        f32::acos(Vector::dot_product(self, rhs) / (self.magnitude() * self.magnitude()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_magnitude() {
        let i_unit_vector: Vector = Vector::new(1_f32, 0_f32, 0_f32);
        let j_unit_vector: Vector = Vector::new(0_f32, 1_f32, 0_f32);
        let k_unit_vector: Vector = Vector::new(0_f32, 0_f32, 1_f32);

        assert_eq!(i_unit_vector.magnitude(), 1_f32);
        assert_eq!(j_unit_vector.magnitude(), 1_f32);
        assert_eq!(k_unit_vector.magnitude(), 1_f32);
    }

    #[test]
    fn unit_cross_product() {
        let i_unit_vector: Vector = Vector::new(1_f32, 0_f32, 0_f32);
        let j_unit_vector: Vector = Vector::new(0_f32, 1_f32, 0_f32);
        let k_unit_vector: Vector = Vector::new(0_f32, 0_f32, 1_f32);

        assert_eq!(i_unit_vector.cross_product(&j_unit_vector), k_unit_vector);
        assert_eq!(j_unit_vector.cross_product(&k_unit_vector), i_unit_vector);
        assert_eq!(k_unit_vector.cross_product(&i_unit_vector), j_unit_vector);

        assert_eq!(
            j_unit_vector.cross_product(&j_unit_vector),
            -(j_unit_vector.cross_product(&j_unit_vector))
        );
    }

    #[test]
    fn unit_unit_scalar() {
        let i_unit_vector: Vector = Vector::new(1_f32, 0_f32, 0_f32);

        assert_eq!((&i_unit_vector * 2_f32).unit(), i_unit_vector)
    }
}
