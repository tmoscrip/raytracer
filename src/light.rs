use crate::{colour::Colour, tuple::Tuple};

#[derive(Clone)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Colour,
}

impl Light {
    pub fn point_light(position: Tuple, intensity: Colour) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = crate::colour::Colour::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = Light::point_light(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
