use std::io::Write;

use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vec3;

pub const SPHERES: [Sphere; 4] = [
    Sphere::new(Vec3::new(-3f32, 0f32, -16f32), 2f32, Material::IVORY),
    Sphere::new(Vec3::new(-1.0f32, -1.5f32, -12f32), 2f32, Material::GLASS),
    Sphere::new(
        Vec3::new(1.5f32, -0.5f32, -18f32),
        3f32,
        Material::RED_RUBBER,
    ),
    Sphere::new(Vec3::new(7f32, 5f32, -18f32), 4f32, Material::MIRROR),
];

pub const LIGHTS: [Vec3; 3] = [
    Vec3::new(-20f32, 20f32, 20f32),
    Vec3::new(30f32, 50f32, -25f32),
    Vec3::new(30f32, 20f32, 30f32),
];

fn scene_intersect(ray: &Ray) -> Option<(Vec3, Vec3, Material)> {
    let mut nearest_dist = 1e10;
    let mut pt = Vec3::default();
    let mut normal = Vec3::default();
    let mut material = Material::default();

    if ray.direction.y.abs() > 0.001f32 {
        let d = -(ray.origin.y + 4f32) / ray.direction.y;
        let p = ray.origin + ray.direction * d;

        if d > 0.001f32 && d < nearest_dist && p.x.abs() < 10f32 && p.z < -10f32 && p.z > -30f32 {
            nearest_dist = d;
            pt = p;
            normal = Vec3::new(0f32, 1f32, 0f32);

            material.diffuse_color =
                if ((0.5f32 * pt.x + 1000f32) as i32 + (0.5f32 * pt.z) as i32) & 1 == 1 {
                    Vec3::new(0.3f32, 0.3f32, 0.3f32)
                } else {
                    Vec3::new(0.3f32, 0.2f32, 0.1f32)
                };
        }
    }

    for sphere in &SPHERES {
        let intersection = match ray.sphere_intersect(sphere) {
            Some(intersection) => intersection,
            None => continue,
        };

        if intersection > nearest_dist {
            continue;
        }

        nearest_dist = intersection;
        pt = ray.origin + ray.direction * nearest_dist;
        normal = (pt - sphere.center).normalized();
        material = sphere.material.clone();
    }

    if nearest_dist >= 1000f32 {
        return None;
    }

    // println!("{:#?}", material.specular_exponent);
    Some((pt, normal, material))
}

fn cast_ray(ray: &Ray, depth: i32) -> Vec3 {
    let (point, normal, material) = match scene_intersect(ray) {
        Some(intersect) => intersect,
        None => return Vec3::new(0.2f32, 0.7f32, 0.8f32),
    };

    if depth > 4 {
        return Vec3::new(0.2f32, 0.7f32, 0.8f32);
    }

    let reflect_dir = ray.direction.reflect(normal).normalized();
    let refract_dir = ray
        .direction
        .refract(normal, material.refractive_index, 1f32)
        .normalized();
    let reflect_color = cast_ray(&Ray::new(point, reflect_dir), depth + 1);
    let refract_color = cast_ray(&Ray::new(point, refract_dir), depth + 1);

    let mut diffuse_light_intensity = 0f32;
    let mut specular_light_intensity = 0f32;

    for light in &LIGHTS {
        let light_direction = (light.clone() - point).normalized();
        if let Some((shadow_pt, _trashnrm, _trashmat)) =
            scene_intersect(&Ray::new(point, light_direction))
        {
            if (shadow_pt - point).norm() < (light.clone() - point).norm() {
                continue;
            }
        }
        diffuse_light_intensity += 0f32.max(light_direction * normal);
        specular_light_intensity += 0f32
            .max(-(-light_direction).reflect(normal) * ray.direction)
            .powf(material.specular_exponent);
    }

    return material.diffuse_color * diffuse_light_intensity * material.albedo[0]
        + Vec3::new(1f32, 1f32, 1f32) * specular_light_intensity * material.albedo[1]
        + reflect_color * material.albedo[2]
        + refract_color * material.albedo[3];
}

pub fn launch() {
    let width = 1024;
    let height = 768;
    let fov = 1.05f32;
    let mut framebuffer = vec![Vec3::default(); width * height];

    for pix in 0..width * height {
        let dir_x = ((pix % width) as f32 + 0.5f32) - (width as f32) / 2f32;
        let dir_y = -((pix / width) as f32 + 0.5f32) + (height as f32) / 2f32;
        let dir_z = -(height as f32) / (2f32 * (fov / 2f32).tan());
        let direction = Vec3::new(dir_x, dir_y, dir_z);
        let ray = Ray::new(Vec3::new(0f32, 0f32, 0f32), direction.normalized());
        framebuffer[pix] = cast_ray(&ray, 0);
    }

    let mut file = std::fs::File::create("./out.ppm").unwrap();

    // Header
    file.write(b"P6\n").unwrap();
    file.write(width.to_string().as_bytes()).unwrap();
    file.write(b" ").unwrap();
    file.write(height.to_string().as_bytes()).unwrap();
    file.write(b"\n255\n").unwrap();

    // Data
    for color in framebuffer {
        let max = 1f32.max(color[0]).max(color[1]).max(color[2]);
        for chan in 0..3 {
            let value = 255f32 * color[chan] / max;
            file.write(&[value as u8]).unwrap();
        }
    }
}
