use crate::{colour::Colour, matrix::Matrix, shape::Shape, tuple::Tuple};

#[derive(Clone)]
pub struct StripePattern {
    a: Colour,
    b: Colour,
    transform: Matrix,
}

impl StripePattern {
    pub fn new(a: Colour, b: Colour) -> Self {
        StripePattern {
            a,
            b,
            transform: Matrix::identity(),
        }
    }

    pub fn stripe_at(&self, point: Tuple) -> Colour {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: &dyn Shape, world_point: Tuple) -> Colour {
        let object_point = object.data().inverse_transform.clone() * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.stripe_at(pattern_point)
    }

    pub fn set_pattern_transform(&mut self, transform: Matrix) {
        self.transform = transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.a, white);
        assert_eq!(pattern.b, black);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(Tuple::point(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(Tuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(Tuple::point(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(Tuple::point(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(Tuple::point(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        use crate::{matrix::Matrix, shape::sphere::Sphere};

        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = StripePattern::new(white, black);

        let c = pattern.stripe_at_object(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        use crate::shape::sphere::Sphere;

        let object = Sphere::new();

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let mut pattern = StripePattern::new(white, black);
        pattern.set_pattern_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let c = pattern.stripe_at_object(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        use crate::{matrix::Matrix, shape::sphere::Sphere};

        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let mut pattern = StripePattern::new(white, black);
        pattern.set_pattern_transform(Matrix::translation(0.5, 0.0, 0.0));

        let c = pattern.stripe_at_object(&object, Tuple::point(2.5, 0.0, 0.0));

        assert_eq!(c, white);
    }
}
