use crate::tuple::Tuple;

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

impl Environment {
    pub fn new(g: Tuple, w: Tuple) -> Environment {
        Environment {
            gravity: g,
            wind: w,
        }
    }
}
