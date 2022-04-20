use crate::material::Material;
use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    #[inline]
    pub const fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
