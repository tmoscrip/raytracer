use crate::{colour::Colour, tuple::Tuple};

#[derive(Clone, Copy)]
pub struct StripePattern {
    a: Colour,
    b: Colour,
}

impl StripePattern {
    pub fn new(a: Colour, b: Colour) -> Self {
        StripePattern { a, b }
    }

    pub fn stripe_at(&self, point: Tuple) -> Colour {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
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
}
