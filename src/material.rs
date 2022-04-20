use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub refractive_index: f32,
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3,
    pub specular_exponent: f32,
}

impl Material {
    pub const IVORY: Material = Material {
        refractive_index: 1.0f32,
        albedo: [0.9f32, 0.5f32, 0.1f32, 0f32],
        diffuse_color: Vec3::new(0.4f32, 0.4f32, 0.3f32),
        specular_exponent: 50f32,
    };

    pub const GLASS: Material = Material {
        refractive_index: 1.5f32,
        albedo: [0f32, 0.9f32, 0.1f32, 0.8f32],
        diffuse_color: Vec3::new(0.6f32, 0.7f32, 0.8f32),
        specular_exponent: 125f32,
    };

    pub const RED_RUBBER: Material = Material {
        refractive_index: 1.0f32,
        albedo: [1.4f32, 0.3f32, 0f32, 0f32],
        diffuse_color: Vec3::new(0.3f32, 0.1f32, 0.1f32),
        specular_exponent: 10f32,
    };

    pub const MIRROR: Material = Material {
        refractive_index: 1.0f32,
        albedo: [0f32, 16f32, 0.8f32, 0f32],
        diffuse_color: Vec3::new(1.0f32, 1.0f32, 1.0f32),
        specular_exponent: 1425f32,
    };
}

impl Default for Material {
    fn default() -> Self {
        Self {
            refractive_index: 1f32,
            albedo: [2f32, 0f32, 0f32, 0f32],
            diffuse_color: Vec3::new(0f32, 0f32, 0f32),
            specular_exponent: 0f32,
        }
    }
}
