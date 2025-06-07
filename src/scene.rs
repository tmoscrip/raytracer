use crate::colour::Colour;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene {
    width: u32,
    height: u32,
    colours: Vec<Colour>,
    buffer: Vec<u8>,
    time: f32,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Scene {
        let pixel_count = (width * height) as usize;
        let colours = vec![Colour::new(0.0, 0.0, 0.0); pixel_count];
        let buffer_size = (width * height * 4) as usize;
        let buffer = vec![0; buffer_size];

        Scene {
            width,
            height,
            colours,
            buffer,
            time: 0.0,
        }
    }

    pub fn render(&mut self, dt: f32) {
        self.time += dt;

        // Update colours array instead of buffer directly
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_index = (y * self.width + x) as usize;

                let r = 0.5 * (1.0 + (self.time + (x as f32 * 0.01)).sin());
                let g = 0.5 * (1.0 + (self.time + (y as f32 * 0.02)).sin());
                let b = 0.5 * (1.0 + (self.time + ((x + y) as f32 * 0.03)).sin());

                self.colours[pixel_index] = Colour::new(r as f64, g as f64, b as f64);
            }
        }

        // Convert colours to buffer for canvas
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

impl Scene {
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
        let scene = Scene::new(width, height);

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
        let mut scene = Scene::new(10, 20);
        let red = Colour::new(1.0, 0.0, 0.0);

        scene.write_pixel(2, 3, red);

        assert_eq!(scene.get_pixel_colour(2, 3), red);
    }
}
