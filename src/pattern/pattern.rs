use crate::{colour::Colour, matrix::Matrix, shape::Shape, tuple::Tuple};

#[derive(Clone)]
pub struct PatternData {
    pub a: Colour,
    pub b: Colour,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
}

pub trait Pattern {
    fn set_transform(&mut self, transform: Matrix) {
        self.data_mut().inverse_transform = transform.inverse();
        self.data_mut().transform = transform;
    }

    fn pattern_at_shape(&self, shape: &dyn Shape, world_point: Tuple) -> Colour {
        let object_point = shape.data().inverse_transform.clone() * world_point;
        let pattern_point = self.data().inverse_transform.clone() * object_point;
        self.pattern_at(pattern_point)
    }

    // Abstract methods
    fn data(&self) -> &PatternData;
    fn data_mut(&mut self) -> &mut PatternData;
    fn pattern_at(&self, point: Tuple) -> Colour;
}

#[cfg(test)]
mod tests {
    use crate::shape::sphere::Sphere;

    use super::*;

    struct TestPattern {
        data: PatternData,
    }

    impl TestPattern {
        pub fn new() -> Self {
            let identity: Matrix = Matrix::identity();
            Self {
                data: PatternData {
                    a: Colour::black(),
                    b: Colour::white(),
                    transform: identity.clone(),
                    inverse_transform: identity.inverse(),
                },
            }
        }
    }

    impl Pattern for TestPattern {
        fn data(&self) -> &PatternData {
            &self.data
        }

        fn data_mut(&mut self) -> &mut PatternData {
            &mut self.data
        }

        fn pattern_at(&self, point: Tuple) -> Colour {
            Colour::new(point.x, point.y, point.z)
        }
    }

    #[test]
    fn pattern_can_be_assigned_transformation() {
        let mut p = TestPattern::new();
        let t = Matrix::translation(1.0, 2.0, 3.0);
        p.set_transform(t.clone());
        assert_eq!(p.data.transform, t);
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let p = TestPattern::new();
        let c = p.pattern_at_shape(&s, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Colour::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let s = Sphere::new();
        let mut p = TestPattern::new();
        p.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let c = p.pattern_at_shape(&s, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Colour::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_both_object_and_pattern_transformation() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let mut p = TestPattern::new();
        p.set_transform(Matrix::translation(0.5, 1.0, 1.5));
        let c = p.pattern_at_shape(&s, Tuple::point(2.5, 3.0, 3.5));
        assert_eq!(c, Colour::new(0.75, 0.5, 0.25));
    }
}
