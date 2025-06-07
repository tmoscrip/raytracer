use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
    time: f32,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Scene {
        let size = (width * height * 4) as usize;
        let buffer = vec![0; size];
        Scene { width, height, buffer, time: 0.0 }
    }

    pub fn render(&mut self, dt: f32) {
        self.time += dt;
        for y in 0..self.height {
            for x in 0..self.width {
                let i = ((y * self.width + x) * 4) as usize;
                let r = (0.5 * (1.0 + (self.time + (x as f32 * 0.01)).sin()) * 255.0) as u8;
                let g = (0.5 * (1.0 + (self.time + (y as f32 * 0.02)).sin()) * 255.0) as u8;
                let b = (0.5 * (1.0 + (self.time + ((x + y) as f32 * 0.03)).sin()) * 255.0) as u8;

                self.buffer[i] = r;
                self.buffer[i + 1] = g;
                self.buffer[i + 2] = b;
                self.buffer[i + 3] = 255; // Alpha
            }
        }
    }

    pub fn get_image_buffer_pointer(&self) -> *const u8 {
        self.buffer.as_ptr()
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
        assert_eq!(scene.buffer.len(), (width * height * 4) as usize, "Buffer should be width * height * 4");
    }
}
