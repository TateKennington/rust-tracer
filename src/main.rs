mod geometry;
mod material;
mod raytracer;
mod scene;

use raytracer::render;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("./dist/image.png").unwrap();
    file.write_all(&render()).unwrap();
}
