#[macro_use]
extern crate impl_ops;

mod geometry;
mod material;
mod raytracer;
mod scene;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render() -> js_sys::Uint8Array {
    let result = raytracer::render(1, 50);
    let buffer = js_sys::Uint8Array::new_with_length(result.len() as u32);
    result
        .iter()
        .enumerate()
        .for_each(|(i, byte)| buffer.set_index(i as u32, *byte));
    return buffer;
}
