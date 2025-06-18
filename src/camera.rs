use crate::{colour::Colour, matrix::Matrix, ray::Ray, tuple::Tuple, world::World};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Colour>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Colour::black(); width * height];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Colour {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            Colour::black()
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = colour;
        }
    }
}

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width: f64;
        let half_height: f64;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let identity = Matrix::identity();
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: identity.clone(),
            inverse_transform: identity,
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / hsize as f64,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.inverse_transform = transform.inverse();
        self.transform = transform;
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset as f64;
        let world_y = self.half_height - yoffset as f64;

        // canvas at -1
        let pixel = self.inverse_transform.clone() * Tuple::point(world_x, world_y, -1.0);
        let origin = self.inverse_transform.clone() * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalise();

        return Ray::new(origin, direction);
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let colour = world.colour_at(&ray, crate::world::MAX_BOUNCES);
                image.write_pixel(x, y, colour);
            }
        }

        image
    }

    pub fn render_to_buffer(&self, world: &World, buffer: &mut [Colour]) {
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let colour = world.colour_at(&ray, crate::world::MAX_BOUNCES);
                let pixel_index = y * self.hsize + x;
                buffer[pixel_index] = colour;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::tuple::Tuple;

    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_abs_diff_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_abs_diff_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(
            r.direction,
            Tuple::vector(0.66519, 0.33259, -0.66851),
            epsilon = 0.0001
        );
    }

    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.set_transform(Matrix::rotation_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);

        assert_abs_diff_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        let sqrt_2_div_2 = (2.0_f64).sqrt() / 2.0;
        assert_abs_diff_eq!(
            r.direction,
            Tuple::vector(sqrt_2_div_2, 0.0, -sqrt_2_div_2),
            epsilon = 0.0001
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        use crate::{colour::Colour, transformations::view_transform, world::World};

        let w = World::default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.set_transform(view_transform(from, to, up));

        let image = c.render(&w);

        assert_abs_diff_eq!(
            image.pixel_at(5, 5),
            Colour::new(0.38066, 0.47583, 0.2855),
            epsilon = 0.0001
        );
    }
}
