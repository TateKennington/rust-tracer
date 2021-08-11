use crate::geometry::hittable::Hittable;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector3::{Color, Vector3};
use crate::geometry::Geometry;
use crate::material::{
    dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material, MaterialKind,
};
use crate::scene::{camera::Camera, Scene};
use rand::prelude::*;
use std::io::{BufWriter, Write};

const aspect_ratio: f64 = 16.0 / 9.0;
const image_width: f64 = 400.0;
const image_height: f64 = image_width / aspect_ratio;
const width: usize = image_width as usize;
const height: usize = image_height as usize;
const focal_length: f64 = 1.0;

pub fn render(samples_per_pixel: usize, max_depth: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(width * height);
    {
        let writer: BufWriter<&mut Vec<_>> = BufWriter::new(buffer.as_mut());
        let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut png_writer = encoder.write_header().unwrap();
        let mut stream = png_writer.stream_writer();

        let camera = Camera::new(
            Vector3::new(3.0, 3.0, 2.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
            aspect_ratio,
            20.0,
            0.1,
            27.0f64.sqrt(),
        );

        let material_ground = Lambertian {
            albedo: Vector3::new(0.8, 0.8, 0.0),
        };
        let material_center = Lambertian {
            albedo: Vector3::new(0.1, 0.2, 0.5),
        };
        let material_left = Dielectric { ir: 1.5 };
        let material_left_inner = Dielectric { ir: 1.5 };
        let material_right = Metal {
            albedo: Vector3::new(0.8, 0.6, 0.2),
            fuzz: 0.0,
        };

        let mut scene = Scene { objects: vec![] };
        scene.add(
            Geometry::Sphere(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
            MaterialKind::Lambertian(material_ground),
        );
        scene.add(
            Geometry::Sphere(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
            MaterialKind::Lambertian(material_center),
        );
        scene.add(
            Geometry::Sphere(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5)),
            MaterialKind::Dielectric(material_left),
        );
        scene.add(
            Geometry::Sphere(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.4)),
            MaterialKind::Dielectric(material_left_inner),
        );
        scene.add(
            Geometry::Sphere(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5)),
            MaterialKind::Metal(material_right),
        );

        let mut rng = thread_rng();
        for y in 0..height {
            println!("\rProgress {}/{}", y + 1, height);
            for x in 0..width {
                let mut pixel = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (rng.gen::<f64>() + x as f64) / width as f64;
                    let v = (rng.gen::<f64>() + y as f64) / height as f64;
                    pixel += ray_color(camera.get_ray(u, v), &scene, max_depth)
                }
                pixel /= samples_per_pixel as f64;
                pixel.sqrt();
                pixel *= 255.0;
                stream
                    .write(&[pixel.x as u8, pixel.y as u8, pixel.z as u8, 255u8])
                    .unwrap();
            }
        }
        stream.finish().unwrap();
    }

    return buffer;
}

fn ray_color(r: Ray, scene: &Scene, depth: usize) -> Color {
    if depth == 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = scene.hit(&r, 0.0001, f64::MAX) {
        if let Some(material) = &hit.material {
            if let Some((attentuation, scatter_ray)) = material.scatter(&r, &hit) {
                let mut color = ray_color(scatter_ray, scene, depth - 1);
                color.x *= attentuation.x;
                color.y *= attentuation.y;
                color.z *= attentuation.z;
                return color;
            }
        }
        return Vector3::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vector3::lerp(
        &Vector3::new(1.0, 1.0, 1.0),
        &Vector3::new(0.5, 0.7, 1.0),
        t,
    );
}
