use crate::{
    colour::Colour,
    matrix::Matrix,
    pattern::{Pattern, PatternData},
    tuple::Tuple,
};

#[derive(Clone)]
pub struct Checkered {
    data: PatternData,
}

impl Pattern for Checkered {
    fn data(&self) -> &PatternData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut PatternData {
        &mut self.data
    }

    fn pattern_at(&self, point: Tuple) -> Colour {
        let sum = point.x.floor() as i32 + point.y.floor() as i32 + point.z.floor() as i32;
        if sum % 2 == 0 {
            self.data().a
        } else {
            self.data().b
        }
    }
}

impl Checkered {
    pub fn new(a: Colour, b: Colour) -> Self {
        let identity: Matrix = Matrix::identity();
        Self {
            data: PatternData {
                a,
                b,
                transform: identity.clone(),
                inverse_transform: identity.inverse(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_checkered_pattern() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Checkered::new(white, black);

        assert_eq!(pattern.data.a, white);
        assert_eq!(pattern.data.b, black);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Checkered::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.99, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(1.01, 0.0, 0.0)), black);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Checkered::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.99, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 1.01, 0.0)), black);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Checkered::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.99)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 1.01)), black);
    }

    #[test]
    fn checkers_pattern_with_object_transformation() {
        use crate::{
            matrix::Matrix,
            shape::{sphere::Sphere, Shape},
        };

        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Checkered::new(white, black);

        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, white);
    }

    #[test]
    fn checkers_pattern_with_pattern_transformation() {
        use crate::shape::sphere::Sphere;

        let object = Sphere::new();

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let mut pattern = Checkered::new(white, black);
        pattern.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, white);
    }
}
