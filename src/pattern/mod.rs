pub mod gradient;
pub mod pattern;
pub mod ring;
pub mod striped;

use crate::{
    colour::Colour,
    pattern::{gradient::Gradient, ring::Ring, striped::Striped},
    shape::Shape,
    tuple::Tuple,
};

pub use pattern::{Pattern, PatternData};

#[derive(Clone)]
pub enum PatternType {
    Striped(Striped),
    Gradient(Gradient),
    Ring(Ring),
}

impl PatternType {
    pub fn pattern_at_shape(&self, shape: &dyn Shape, world_point: Tuple) -> Colour {
        match self {
            PatternType::Striped(pattern) => pattern.pattern_at_shape(shape, world_point),
            PatternType::Gradient(pattern) => pattern.pattern_at_shape(shape, world_point),
            PatternType::Ring(pattern) => pattern.pattern_at_shape(shape, world_point),
        }
    }
}
