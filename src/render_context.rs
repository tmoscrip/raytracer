use crate::{
    colour::Colour,
    matrix::Matrix,
    ray::{self, Ray},
    sphere::{intersect, Sphere},
    tuple::Tuple,
};
use wasm_bindgen::prelude::*;

pub struct TraceSphereScene {
    sphere: Sphere,
}

impl TraceSphereScene {
    pub fn new() -> Self {
        let mut s = Sphere::new();
        s.set_transform(
            Matrix::translation(0.0, 0.0, 0.0) * Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        );
        Self { sphere: s }
    }
}

#[wasm_bindgen]
pub struct RenderContext {
    width: u32,
    height: u32,
    colours: Vec<Colour>,
    buffer: Vec<u8>,
    time: f32,
    scene: TraceSphereScene,
}

#[wasm_bindgen]
impl RenderContext {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> RenderContext {
        let pixel_count = (width * height) as usize;
        let colours = vec![Colour::new(0.0, 0.0, 0.0); pixel_count];
        let buffer_size = (width * height * 4) as usize;
        let buffer = vec![0; buffer_size];

        let context = RenderContext {
            width,
            height,
            colours,
            buffer,
            time: 0.0,
            scene: TraceSphereScene::new(),
        };

        context
    }

    pub fn render(&mut self, dt: f32) {
        self.time += dt;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_index = (y * self.width + x) as usize;
                self.colours[pixel_index] = Colour::new(0.0, 0.0, 0.0); // Black background
            }
        }

        let ray_origin = Tuple::point(0.0, 0.0, -10.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let pixel_size = wall_size / self.height as f64;
        let half = wall_size / 2.0;

        for y in 0..self.height {
            let world_y = half - pixel_size * y as f64;
            for x in 0..self.width {
                let world_x = half - pixel_size * x as f64;

                let pixel_index = (y * self.width + x) as usize;
                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalise());
                let xs = intersect(&self.scene.sphere, &r);

                if !xs.is_empty() {
                    self.colours[pixel_index] = Colour::new(1.0, 0.0, 0.0); // Red for hit
                }
            }
        }

        self.update_buffer_from_colours();
    }

    pub fn get_image_buffer_pointer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    // Helper method to convert colours to buffer
    fn update_buffer_from_colours(&mut self) {
        for (i, colour) in self.colours.iter().enumerate() {
            let buffer_index = i * 4;

            // Clamp colour values to [0, 1] and convert to [0, 255]
            let r = (colour.r.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (colour.g.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (colour.b.clamp(0.0, 1.0) * 255.0) as u8;

            self.buffer[buffer_index] = r;
            self.buffer[buffer_index + 1] = g;
            self.buffer[buffer_index + 2] = b;
            self.buffer[buffer_index + 3] = 255; // Alpha
        }
    }
}

impl RenderContext {
    pub fn write_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        if x < self.width && y < self.height {
            let pixel_index = (y * self.width + x) as usize;
            self.colours[pixel_index] = colour;

            // Update the corresponding buffer pixels
            let buffer_index = pixel_index * 4;
            let r = (colour.r.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (colour.g.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (colour.b.clamp(0.0, 1.0) * 255.0) as u8;

            self.buffer[buffer_index] = r;
            self.buffer[buffer_index + 1] = g;
            self.buffer[buffer_index + 2] = b;
            self.buffer[buffer_index + 3] = 255; // Alpha
        }
    }

    pub fn get_pixel_colour(&self, x: u32, y: u32) -> Colour {
        if x < self.width && y < self.height {
            let pixel_index = (y * self.width + x) as usize;
            self.colours[pixel_index]
        } else {
            Colour::new(0.0, 0.0, 0.0) // Return black for out-of-bounds
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_new() {
        let width = 10;
        let height = 20;
        let scene = RenderContext::new(width, height);

        assert_eq!(scene.width, width);
        assert_eq!(scene.height, height);
        assert_eq!(
            scene.colours.len(),
            (width * height) as usize,
            "Colours array should be width * height"
        );
        assert_eq!(
            scene.buffer.len(),
            (width * height * 4) as usize,
            "Buffer should be width * height * 4"
        );

        for colour in &scene.colours {
            assert_eq!(colour.r, 0.0);
            assert_eq!(colour.g, 0.0);
            assert_eq!(colour.b, 0.0);
        }
    }

    #[test]
    fn write_pixel_to_scene() {
        let mut scene = RenderContext::new(10, 20);
        let red = Colour::new(1.0, 0.0, 0.0);

        scene.write_pixel(2, 3, red);

        assert_eq!(scene.get_pixel_colour(2, 3), red);
    }
}
