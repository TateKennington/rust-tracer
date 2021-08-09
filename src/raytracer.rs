use crate::geometry::hittable::Hittable;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector3::{Color, Vector3};
use crate::geometry::Geometry;
use crate::material::{lambertian::Lambertian, metal::Metal, Material, MaterialKind};
use crate::scene::Scene;
use rand::prelude::*;
use std::io::{BufWriter, Write};

const aspect_ratio: f64 = 16.0 / 9.0;
const image_width: f64 = 400.0;
const image_height: f64 = image_width / aspect_ratio;
const width: usize = image_width as usize;
const height: usize = image_height as usize;
const samples_per_pixel: usize = 1;
const max_depth: usize = 50;

const viewport_height: f64 = 2.0;
const viewport_width: f64 = aspect_ratio * viewport_height;
const focal_length: f64 = 1.0;

pub fn render() -> Vec<u8> {
    let mut buffer = Vec::with_capacity(width * height);
    {
        let writer: BufWriter<&mut Vec<_>> = BufWriter::new(buffer.as_mut());
        let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut png_writer = encoder.write_header().unwrap();
        let mut stream = png_writer.stream_writer();

        let horizontal = &Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };

        let vertical = &Vector3 {
            x: 0.0,
            y: -viewport_height,
            z: 0.0,
        };

        let viewport_origin = Vector3 {
            x: -viewport_width / 2.0,
            y: viewport_height / 2.0,
            z: -focal_length,
        };

        let material_ground = Lambertian {
            albedo: Vector3 {
                x: 0.8,
                y: 0.8,
                z: 0.0,
            },
        };
        let material_center = Lambertian {
            albedo: Vector3 {
                x: 0.7,
                y: 0.3,
                z: 0.3,
            },
        };
        let material_left = Metal {
            albedo: Vector3 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
        };
        let material_right = Metal {
            albedo: Vector3 {
                x: 0.8,
                y: 0.6,
                z: 0.2,
            },
        };

        let mut scene = Scene { objects: vec![] };
        scene.add(
            Geometry::Sphere(Sphere {
                origin: Vector3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                radius: 100.0,
            }),
            MaterialKind::Lambertian(material_ground),
        );
        scene.add(
            Geometry::Sphere(Sphere {
                origin: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            }),
            MaterialKind::Lambertian(material_center),
        );
        scene.add(
            Geometry::Sphere(Sphere {
                origin: Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            }),
            MaterialKind::Metal(material_left),
        );
        scene.add(
            Geometry::Sphere(Sphere {
                origin: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            }),
            MaterialKind::Metal(material_right),
        );

        let mut rng = thread_rng();
        for y in 0..height {
            println!("\rProgress {}/{}", y + 1, height);
            for x in 0..width {
                let mut pixel = Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for _ in 0..samples_per_pixel {
                    let u = (rng.gen::<f64>() + x as f64) / width as f64;
                    let v = (rng.gen::<f64>() + y as f64) / height as f64;
                    let direction = u * horizontal + v * vertical + viewport_origin;
                    pixel += ray_color(
                        Ray {
                            origin: Vector3 {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            direction,
                        },
                        &scene,
                        0,
                    )
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
    if depth >= max_depth {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if let Some(hit) = scene.hit(&r, 0.0001, f64::MAX) {
        if let Some(material) = &hit.material {
            if let Some((attentuation, scatter_ray)) = material.scatter(&r, &hit) {
                let mut color = ray_color(scatter_ray, scene, depth + 1);
                color.x *= attentuation.x;
                color.y *= attentuation.y;
                color.z *= attentuation.z;
                return color;
            }
        }
        return Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vector3::lerp(
        &Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        &Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        },
        t,
    );
}
