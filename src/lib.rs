pub mod camera;
pub mod colour;
pub mod environment;
pub mod intersection;
pub mod light;
pub mod materials;
pub mod matrix;
pub mod projectile;
pub mod ray;
pub mod render_context;
pub mod shape;
pub mod simulation;
pub mod sphere_registry;
pub mod transformations;
pub mod tuple;
pub mod world;

// Add a simple performance timing utility
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

#[cfg(target_arch = "wasm32")]
pub fn console_log(s: &str) {
    log(s);
}

#[cfg(target_arch = "wasm32")]
pub fn performance_now() -> f64 {
    now()
}
