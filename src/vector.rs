//! Vector component

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(unused)]
impl Vec3 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        self.clone() * (1f32 / self.norm())
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn cross_assign(&mut self, rhs: Self) {
        *self = self.cross(rhs);
    }

    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2f32 * (self * normal)
    }

    #[inline]
    pub fn refract(self, normal: Self, eta_t: f32, eta_i: f32) -> Self {
        let cosi = -(self * normal).clamp(-1f32, 1f32);
        if cosi < 0f32 {
            return self.refract(-normal, eta_i, eta_t);
        }

        let eta = eta_i / eta_t;
        let k = 1f32 - eta * eta * (1f32 - cosi * cosi);

        if k < 0f32 {
            return Vec3::new(1f32, 0f32, 0f32);
        }

        self * eta + normal * (eta * cosi - k.sqrt())
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl Mul for Vec3 {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(mut self, rhs: f32) -> Vec3 {
        self *= rhs;
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index ouf of vector range: {index}"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index ouf of vector range: {index}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_new() {
        let vector = Vec3::new(1f32, 2f32, 3f32);
        assert_eq!(vector.x, 1f32);
        assert_eq!(vector.y, 2f32);
        assert_eq!(vector.z, 3f32);
    }

    #[test]
    fn test_add() {
        let vector1 = Vec3::new(1f32, 2f32, 3f32);
        let vector2 = Vec3::new(1f32, 2f32, 3f32);

        let vector3 = vector1 + vector2;

        assert_eq!(vector3.x, 2f32);
        assert_eq!(vector3.y, 4f32);
        assert_eq!(vector3.z, 6f32);
    }

    #[test]
    fn test_sub() {
        let vector1 = Vec3::new(6f32, 12f32, 18f32);
        let vector2 = Vec3::new(1f32, 2f32, 3f32);

        let vector3 = vector1 - vector2;

        assert_eq!(vector3.x, 5f32);
        assert_eq!(vector3.y, 10f32);
        assert_eq!(vector3.z, 15f32);
    }

    #[test]
    fn test_mul() {
        let vector1 = Vec3::new(6f32, 12f32, 18f32);
        let vector2 = Vec3::new(2f32, 3f32, 4f32);

        assert_eq!(vector1 * vector2, 120f32);
    }

    #[test]
    fn test_indexl() {
        let vector = Vec3::new(6f32, 12f32, 18f32);

        assert_eq!(vector[0], 6f32);
        assert_eq!(vector[1], 12f32);
        assert_eq!(vector[2], 18f32);
    }
}
