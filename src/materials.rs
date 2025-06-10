use crate::{
    colour::Colour,
    light::Light,
    tuple::{reflect, Tuple},
};

#[derive(Clone)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            colour: Colour::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn colour(&self) -> &Colour {
        &self.colour
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    // Setters
    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse;
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular;
    }

    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = shininess;
    }
}

pub fn lighting(
    material: Material,
    light: Light,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    in_shadow: bool,
) -> Colour {
    let effective_colour = material.colour * light.intensity;
    let lightv = (light.position - point).normalise();
    let ambient = effective_colour * material.ambient;
    let light_dot_normal = lightv.dot(&normalv);

    let specular: Colour;
    let diffuse: Colour;
    if light_dot_normal < 0.0 || in_shadow {
        diffuse = Colour::black();
        specular = Colour::black();
    } else {
        diffuse = effective_colour * material.diffuse * light_dot_normal;
        let reflectv = reflect(&(-lightv), &normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);

        if reflect_dot_eye <= 0.0 {
            specular = Colour::black();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    return ambient + diffuse + specular;
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new();

        assert_eq!(m.colour, Colour::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(Tuple::point(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let in_shadow = false;

        let result = lighting(m, light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let sqrt_2_div_2 = (2.0_f64).sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, sqrt_2_div_2, -sqrt_2_div_2);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(Tuple::point(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let in_shadow = false;

        let result = lighting(m, light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Colour::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(Tuple::point(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let in_shadow = false;

        let result = lighting(m, light, position, eyev, normalv, in_shadow);

        assert_abs_diff_eq!(
            result,
            Colour::new(0.7364, 0.7364, 0.7364),
            epsilon = 0.0001
        );
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let sqrt_2_div_2 = (2.0_f64).sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, -sqrt_2_div_2, -sqrt_2_div_2);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(Tuple::point(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let in_shadow = false;

        let result = lighting(m, light, position, eyev, normalv, in_shadow);

        assert_abs_diff_eq!(
            result,
            Colour::new(1.6364, 1.6364, 1.6364),
            epsilon = 0.0001
        );
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(Tuple::point(0.0, 0.0, 10.0), Colour::new(1.0, 1.0, 1.0));
        let in_shadow = false;

        let result = lighting(m, light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(Tuple::point(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let in_shadow = true;

        let result = lighting(m, light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
    }
}
