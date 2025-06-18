pub mod pattern;
pub mod striped;
use crate::{colour::Colour, pattern::striped::Striped, shape::Shape, tuple::Tuple};
pub use pattern::{Pattern, PatternData};

#[derive(Clone)]
pub enum PatternType {
    Striped(Striped),
    Gradient(Gradient),
}

impl PatternType {
    pub fn pattern_at_shape(&self, shape: &dyn Shape, world_point: Tuple) -> Colour {
        match self {
            PatternType::Striped(pattern) => pattern.pattern_at_shape(shape, world_point),
        }
    }
}
