use crate::{
    colour::Colour,
    matrix::Matrix,
    pattern::{Pattern, PatternData},
    tuple::Tuple,
};

#[derive(Clone)]
pub struct Gradient {
    data: PatternData,
}

impl Pattern for Gradient {
    fn data(&self) -> &PatternData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut PatternData {
        &mut self.data
    }

    fn pattern_at(&self, point: Tuple) -> Colour {
        let a = self.data().a;
        let b = self.data().b;

        let dist = b - a;
        let frac = point.x - point.x.floor();

        a + (dist * frac)
    }
}

impl Gradient {
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
    fn a_gradient_linearly_interpolates_between_colors() {
        let white = Colour::new(1.0, 1.0, 1.0);
        let black = Colour::new(0.0, 0.0, 0.0);
        let pattern = Gradient::new(white, black);

        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.25, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.75, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }
}
