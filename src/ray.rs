use crate::{sphere::Sphere, vector::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[allow(unused)]
impl Ray {
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[inline]
    pub fn sphere_intersect(&self, sphere: &Sphere) -> Option<f32> {
        let length = sphere.center - self.origin;
        let tca = length * self.direction;
        let d2 = length * length - tca * tca;
        if d2 > sphere.radius * sphere.radius {
            return None;
        }

        let thc = (sphere.radius * sphere.radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        if t0 > 0.001f32 {
            return Some(t0);
        }
        if t1 > 0.001f32 {
            return Some(t1);
        }

        None
    }

    #[inline]
    pub fn diverge(mut self, direction: Vec3) -> Self {
        self.direction = direction;
        self
    }
}
