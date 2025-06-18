use crate::{
    colour::Colour,
    matrix::Matrix,
    pattern::{Pattern, PatternData},
    tuple::Tuple,
};

#[derive(Clone)]
pub struct Ring {
    data: PatternData,
}

impl Pattern for Ring {
    fn data(&self) -> &PatternData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut PatternData {
        &mut self.data
    }

    fn pattern_at(&self, point: Tuple) -> Colour {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2.0 == 0.0 {
            self.data().a
        } else {
            self.data().b
        }
    }
}

impl Ring {
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
    fn ring_should_extend_in_both_x_and_z() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Ring::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)), black);
        assert_eq!(pattern.pattern_at(Tuple::point(0.708, 0.0, 0.708)), black);
    }
}
