use crate::{
    colour::Colour,
    intersection,
    light::{self, Light},
    materials::{self, lighting},
    matrix::Matrix,
    ray::Ray,
    sphere::{self, intersect, Sphere},
    tuple::Tuple,
};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::{console_log, performance_now};

pub struct TraceSphereScene {
    sphere: Sphere,
    light: Light,
}

impl TraceSphereScene {
    pub fn new() -> Self {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0.0, 0.0, 0.0));
        s.material.colour = Colour::new(1.0, 0.2, 1.0);

        let light_pos = Tuple::point(10.0, 10.0, -10.0);
        let light_colour = Colour::white();
        Self {
            sphere: s,
            light: Light::point_light(light_pos, light_colour),
        }
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

        let context = RenderContext {
            width,
            height,
            colours,
            buffer,
            time: 0.0,
            scene: TraceSphereScene::new(),
            tile_buffer: Vec::new(),
        };

        context
    }

    pub fn update_light_position(&mut self, x: f64, y: f64, z: f64) {
        self.scene.light.position = Tuple::point(x, y, z);
    }

    pub fn update_sphere_position(&mut self, x: f64, y: f64, z: f64) {
        self.scene
            .sphere
            .set_transform(Matrix::translation(x, y, z));
    }

    pub fn render(&mut self, dt: f32) {
        #[cfg(target_arch = "wasm32")]
        let start_time = performance_now();

        self.time += dt;

        // Clear the color buffer
        for color in &mut self.colours {
            *color = Colour::new(0.0, 0.0, 0.0);
        }

        #[cfg(target_arch = "wasm32")]
        let clear_time = performance_now();

        // Pre-calculate constants to avoid repeated calculations
        let ray_origin = Tuple::point(0.0, 0.0, -10.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let pixel_size = wall_size / self.height as f64;
        let half = wall_size / 2.0;

        #[cfg(target_arch = "wasm32")]
        let mut intersection_time = 0.0;
        #[cfg(target_arch = "wasm32")]
        let mut lighting_time = 0.0;
        #[cfg(target_arch = "wasm32")]
        let mut ray_count = 0;

        for y in 0..self.height {
            let world_y = half - pixel_size * y as f64;
            let y_offset = (y * self.width) as usize;

            for x in 0..self.width {
                let world_x = half - pixel_size * x as f64;
                let pixel_index = y_offset + x as usize;

                let position = Tuple::point(world_x, world_y, wall_z);
                let direction = (position - ray_origin).normalise();
                let r = Ray::new(ray_origin, direction);

                #[cfg(target_arch = "wasm32")]
                let intersect_start = performance_now();

                let xs = intersect(&self.scene.sphere, &r);
                let hit = intersection::hit(&xs);

                #[cfg(target_arch = "wasm32")]
                {
                    intersection_time += performance_now() - intersect_start;
                    ray_count += 1;
                }

                if let Some(hit_intersection) = hit {
                    #[cfg(target_arch = "wasm32")]
                    let lighting_start = performance_now();

                    let point = r.position(hit_intersection.t);
                    let normalv = sphere::normal_at(&self.scene.sphere, &point);
                    let eyev = -r.direction;

                    let colour = materials::lighting(
                        self.scene.sphere.material.clone(),
                        self.scene.light.clone(),
                        point,
                        eyev,
                        normalv,
                    );

                    self.colours[pixel_index] = colour;

                    #[cfg(target_arch = "wasm32")]
                    {
                        lighting_time += performance_now() - lighting_start;
                    }
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        let raytracing_time = performance_now();

        self.update_buffer_from_colours();

        #[cfg(target_arch = "wasm32")]
        {
            let total_time = performance_now() - start_time;
            let buffer_time = performance_now() - raytracing_time;

            console_log(&format!(
                "Render breakdown: Total: {:.2}ms, Clear: {:.2}ms, Intersections: {:.2}ms, Lighting: {:.2}ms, Buffer: {:.2}ms, Rays: {}",
                total_time,
                clear_time - start_time,
                intersection_time,
                lighting_time,
                buffer_time,
                ray_count
            ));
        }
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

    // New method for chunked rendering - renders a specific tile
    pub fn render_tile(
        &mut self,
        tile_x: u32,
        tile_y: u32,
        tile_width: u32,
        tile_height: u32,
        full_width: u32,
        full_height: u32,
    ) -> Vec<u8> {
        #[cfg(target_arch = "wasm32")]
        let start_time = performance_now();

        // Pre-calculate constants to avoid repeated calculations
        let ray_origin = Tuple::point(0.0, 0.0, -10.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let pixel_size = wall_size / full_height as f64;
        let half = wall_size / 2.0;

        // Create a buffer for this tile only
        let tile_buffer_size = (tile_width * tile_height * 4) as usize;
        let mut tile_buffer = vec![0u8; tile_buffer_size];

        #[cfg(target_arch = "wasm32")]
        let mut ray_count = 0;

        for local_y in 0..tile_height {
            let global_y = tile_y + local_y;
            let world_y = half - pixel_size * global_y as f64;

            for local_x in 0..tile_width {
                let global_x = tile_x + local_x;
                let world_x = half - pixel_size * global_x as f64;

                let position = Tuple::point(world_x, world_y, wall_z);
                let direction = (position - ray_origin).normalise();
                let r = Ray::new(ray_origin, direction);

                let xs = intersect(&self.scene.sphere, &r);
                let hit = intersection::hit(&xs);

                #[cfg(target_arch = "wasm32")]
                {
                    ray_count += 1;
                }

                let colour = if let Some(hit_intersection) = hit {
                    let point = r.position(hit_intersection.t);
                    let normalv = sphere::normal_at(&self.scene.sphere, &point);
                    let eyev = -r.direction;

                    materials::lighting(
                        self.scene.sphere.material.clone(),
                        self.scene.light.clone(),
                        point,
                        eyev,
                        normalv,
                    )
                } else {
                    Colour::new(0.0, 0.0, 0.0)
                };

                // Write to tile buffer
                let tile_pixel_index = (local_y * tile_width + local_x) as usize;
                let buffer_index = tile_pixel_index * 4;

                let r = (colour.r.clamp(0.0, 1.0) * 255.0) as u8;
                let g = (colour.g.clamp(0.0, 1.0) * 255.0) as u8;
                let b = (colour.b.clamp(0.0, 1.0) * 255.0) as u8;

                tile_buffer[buffer_index] = r;
                tile_buffer[buffer_index + 1] = g;
                tile_buffer[buffer_index + 2] = b;
                tile_buffer[buffer_index + 3] = 255; // Alpha
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let total_time = performance_now() - start_time;
            // console_log(&format!(
            //     "Tile render: {}x{} at ({},{}) - {:.2}ms, {} rays",
            //     tile_width, tile_height, tile_x, tile_y, total_time, ray_count
            // ));
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

    // Get pointer to the tile buffer for JavaScript access
    pub fn get_tile_buffer_pointer(&self) -> *const u8 {
        self.tile_buffer.as_ptr()
    }

    // Get the size of the tile buffer
    pub fn get_tile_buffer_size(&self) -> usize {
        self.tile_buffer.len()
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
