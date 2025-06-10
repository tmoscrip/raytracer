use crate::{camera::Camera, colour::Colour, tuple::Tuple, world::World};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RenderContext {
    width: u32,
    height: u32,
    colours: Vec<Colour>,
    buffer: Vec<u8>,
    world: World,
    camera: Camera,
    tile_buffer: Vec<u8>,
}

#[wasm_bindgen]
impl RenderContext {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> RenderContext {
        let pixel_count = (width * height) as usize;
        let colours = vec![Colour::new(0.0, 0.0, 0.0); pixel_count];
        let buffer_size = (width * height * 4) as usize;
        let buffer = vec![0; buffer_size];

        let mut camera = Camera::new(width as usize, height as usize, std::f64::consts::PI / 3.0);
        let from = Tuple::point(0.0, 1.5, -5.0);
        let to = Tuple::point(0.0, 1.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        camera.set_transform(crate::transformations::view_transform(from, to, up));

        let context = RenderContext {
            width,
            height,
            colours,
            buffer,
            world: World::test_world(),
            camera,
            tile_buffer: Vec::new(),
        };

        context
    }

    pub fn render(&mut self, _dt: f32) {
        for color in &mut self.colours {
            *color = Colour::new(0.0, 0.0, 0.0);
        }

        self.camera.render_to_buffer(&self.world, &mut self.colours);
        self.update_buffer_from_colours();
    }

    pub fn get_image_buffer_pointer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    fn update_buffer_from_colours(&mut self) {
        // Process pixels in chunks for better cache locality
        for (i, &colour) in self.colours.iter().enumerate() {
            let buffer_index = i * 4;

            self.buffer[buffer_index] = (colour.r.clamp(0.0, 1.0) * 255.0) as u8; // R
            self.buffer[buffer_index + 1] = (colour.g.clamp(0.0, 1.0) * 255.0) as u8; // G
            self.buffer[buffer_index + 2] = (colour.b.clamp(0.0, 1.0) * 255.0) as u8; // B
            self.buffer[buffer_index + 3] = 255; // Alpha
        }
    }

    // Chunked rendering - renders a specific tile
    pub fn render_tile(
        &mut self,
        tile_x: u32,
        tile_y: u32,
        tile_width: u32,
        tile_height: u32,
        _full_width: u32,
        _full_height: u32,
    ) -> Vec<u8> {
        // For tile rendering, we can still use the camera's ray_for_pixel method
        // but render only the specific tile region
        let tile_buffer_size = (tile_width * tile_height * 4) as usize;
        let mut tile_buffer = vec![0u8; tile_buffer_size];

        for local_y in 0..tile_height {
            let global_y = tile_y + local_y;

            for local_x in 0..tile_width {
                let global_x = tile_x + local_x;

                let ray = self
                    .camera
                    .ray_for_pixel(global_x as usize, global_y as usize);
                let colour = self.world.colour_at(&ray);

                let tile_pixel_index = (local_y * tile_width + local_x) as usize;
                let buffer_index = tile_pixel_index * 4;

                let r = (colour.r.clamp(0.0, 1.0) * 255.0) as u8;
                let g = (colour.g.clamp(0.0, 1.0) * 255.0) as u8;
                let b = (colour.b.clamp(0.0, 1.0) * 255.0) as u8;

                tile_buffer[buffer_index] = r;
                tile_buffer[buffer_index + 1] = g;
                tile_buffer[buffer_index + 2] = b;
                tile_buffer[buffer_index + 3] = 255;
            }
        }

        tile_buffer
    }

    // Render a tile and store it in the instance for memory access
    pub fn render_tile_and_store(
        &mut self,
        tile_x: u32,
        tile_y: u32,
        tile_width: u32,
        tile_height: u32,
        full_width: u32,
        full_height: u32,
    ) {
        self.tile_buffer = self.render_tile(
            tile_x,
            tile_y,
            tile_width,
            tile_height,
            full_width,
            full_height,
        );
    }

    pub fn get_tile_buffer_pointer(&self) -> *const u8 {
        self.tile_buffer.as_ptr()
    }

    pub fn get_tile_buffer_size(&self) -> usize {
        self.tile_buffer.len()
    }
}

impl RenderContext {
    pub fn write_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        if x < self.width && y < self.height {
            let pixel_index = (y * self.width + x) as usize;
            self.colours[pixel_index] = colour;

            let buffer_index = pixel_index * 4;
            let r = (colour.r.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (colour.g.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (colour.b.clamp(0.0, 1.0) * 255.0) as u8;

            self.buffer[buffer_index] = r;
            self.buffer[buffer_index + 1] = g;
            self.buffer[buffer_index + 2] = b;
            self.buffer[buffer_index + 3] = 255;
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
