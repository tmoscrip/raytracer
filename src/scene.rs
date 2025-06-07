use crate::colour::Colour;
use crate::environment::Environment;
use crate::projectile::Projectile;
use crate::simulation::Simulation;
use crate::tuple::Tuple;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene {
    width: u32,
    height: u32,
    colours: Vec<Colour>,
    buffer: Vec<u8>,
    time: f32,
    // Simulation fields
    simulation: Simulation,
    tick_count: u32,
    max_ticks: u32,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Scene {
        let pixel_count = (width * height) as usize;
        let colours = vec![Colour::new(0.0, 0.0, 0.0); pixel_count];
        let buffer_size = (width * height * 4) as usize;
        let buffer = vec![0; buffer_size];

        let mut scene = Scene {
            width,
            height,
            colours,
            buffer,
            time: 0.0,
            simulation: Simulation::new(
                Environment::new(Tuple::vector(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 0.0)),
                vec![],
            ),
            tick_count: 0,
            max_ticks: 100,
        };

        scene.reset_simulation();
        scene
    }

    pub fn render(&mut self, dt: f32) {
        self.time += dt;

        // Clear background to black for better visibility of projectile
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_index = (y * self.width + x) as usize;
                self.colours[pixel_index] = Colour::new(0.0, 0.0, 0.0); // Black background
            }
        }

        // Run simulation tick and reset when reaching max ticks for looping
        if self.tick_count < self.max_ticks {
            self.simulation.tick();
            self.tick_count += 1;
        } else {
            // Reset simulation for continuous looping
            self.reset_simulation();
        }

        // Draw projectile as red dot only if within viewport bounds
        let projectiles = self.simulation.get_projectiles();
        if !projectiles.is_empty() {
            let projectile = &projectiles[0];

            // Check if projectile is within viewport bounds before drawing
            if projectile.pos.x >= 0.0
                && projectile.pos.x < self.width as f64
                && projectile.pos.y >= 0.0
                && projectile.pos.y < self.height as f64
            {
                let x = projectile.pos.x as u32;
                let y = (self.height as f64 - projectile.pos.y) as u32; // Flip Y coordinate for screen space

                // Draw a larger red dot (5x5 pixels) for better visibility
                for dy in 0..5 {
                    for dx in 0..5 {
                        let px = x + dx;
                        let py = y + dy;
                        if px < self.width && py < self.height {
                            self.write_pixel(px, py, Colour::new(1.0, 0.0, 0.0));
                            // Red dot
                        }
                    }
                }
            }
        }

        // Convert colours to buffer for canvas
        self.update_buffer_from_colours();
    }

    pub fn get_image_buffer_pointer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn reset(&mut self) {
        self.reset_simulation();
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

    pub fn reset_simulation(&mut self) {
        // Reset the simulation to initial state
        let gravity = Tuple::vector(0.0, -0.25, 0.0);
        let wind = Tuple::vector(-0.15, 0.0, 0.0); // Wind for clearer demonstration
        let environment = Environment::new(gravity, wind);

        // Reset projectile to starting position with moderate upward velocity
        let start_pos = Tuple::point(self.width as f64 * 0.1, self.height as f64 * 0.2, 0.0);
        let velocity = Tuple::vector(self.width as f64 * 0.02, self.height as f64 * 0.025, 0.0); // Much slower velocity
        let projectile = Projectile::new(start_pos, velocity);

        self.simulation = Simulation::new(environment, vec![projectile]);
        self.tick_count = 0;
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
