use crate::geometry::Tuple;

pub struct Projectile {
    pub pos: Tuple,
    pub vel: Tuple,
}

impl Projectile {
    pub fn new(p: Tuple, v: Tuple) -> Projectile {
        Projectile { pos: p, vel: v }
    }
}
