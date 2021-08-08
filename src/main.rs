mod geometry;

use geometry::vector3::Vector3;
use std::fs::File;
use std::io::Write;

const aspect_ratio: f64 = 16.0 / 9.0;
const image_width: f64 = 400.0;
const image_height: f64 = image_width / aspect_ratio;
const width: usize = image_width as usize;
const height: usize = image_height as usize;
const samples_per_pixel: usize = 100;
const max_depth: usize = 50;

const viewport_height: f64 = 2.0;
const viewport_width: f64 = aspect_ratio * viewport_height;
const focal_length: f64 = 1.0;

fn main() {
    let mut image: Vec<Vec<Vector3>> = Vec::default();
    for x in 0..width {
        image.push(Vec::with_capacity(height));
        for y in 0..height {
            image[x].push(Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
        }
    }

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
        z: focal_length,
    };

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;

            let direction = u * horizontal + v * vertical + viewport_origin;
            image[x][y] += ray_color(direction)
        }
    }

    let mut output = File::create("./dist/image.ppm").unwrap();
    write!(&mut output, "P3\n{} {}\n255\n", width, height);
    for y in 0..height {
        write!(&mut output, "{}", image[0][y] * 255.0);
        for x in 1..width {
            write!(&mut output, " {}", image[x][y] * 255.0);
        }
        write!(&mut output, "\n");
    }
}

fn ray_color(r: Vector3) -> Vector3 {
    let unit_direction = r.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vector3::lerp(
        &Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        &Vector3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        },
        t,
    );
}
