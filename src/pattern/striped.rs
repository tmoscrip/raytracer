use crate::{
    colour::Colour,
    matrix::Matrix,
    pattern::{Pattern, PatternData},
    tuple::Tuple,
};

#[derive(Clone)]
pub struct Striped {
    data: PatternData,
}

impl Pattern for Striped {
    fn data(&self) -> &PatternData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut PatternData {
        &mut self.data
    }

    fn pattern_at(&self, point: Tuple) -> Colour {
        if point.x.floor() as i32 % 2 == 0 {
            self.data().a
        } else {
            self.data().b
        }
    }
}

impl Striped {
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
    use crate::shape::Shape;

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Striped::new(white, black);

        assert_eq!(pattern.data.a, white);
        assert_eq!(pattern.data.b, black);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Striped::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Striped::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Striped::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(Tuple::point(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(Tuple::point(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(Tuple::point(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        use crate::{matrix::Matrix, shape::sphere::Sphere};

        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Striped::new(white, black);

        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        use crate::shape::sphere::Sphere;

        let object = Sphere::new();

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let mut pattern = Striped::new(white, black);
        pattern.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        use crate::{matrix::Matrix, shape::sphere::Sphere};

        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let mut pattern = Striped::new(white, black);
        pattern.set_transform(Matrix::translation(0.5, 0.0, 0.0));

        let c = pattern.pattern_at_shape(&object, Tuple::point(2.5, 0.0, 0.0));

        assert_eq!(c, white);
    }
}
