use std::ops::{Add, Neg, Sub};

pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64, // 1 for point, 0 for vector
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_point_is_point() {
        let tuple = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(tuple.is_point(), true);
        assert_eq!(tuple.is_vector(), false);
    }

    #[test]
    fn tuple_vector_is_vector() {
        let tuple = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 0.0,
        };
        assert_eq!(tuple.is_point(), false);
        assert_eq!(tuple.is_vector(), true);
    }

    #[test]
    fn point_constructor_returns_point() {
        let tuple = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(tuple.x, 1.0);
        assert_eq!(tuple.y, 2.0);
        assert_eq!(tuple.z, 3.0);
        assert_eq!(tuple.w, 1.0);
    }

    #[test]
    fn vector_constructor_returns_vector() {
        let tuple = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(tuple.x, 1.0);
        assert_eq!(tuple.y, 2.0);
        assert_eq!(tuple.z, 3.0);
        assert_eq!(tuple.w, 0.0);
    }

    #[test]
    fn add_two_tuples() {
        let tuple1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let tuple2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        let tuple3 = tuple1 + tuple2;
        assert_eq!(tuple3.x, 1.0);
        assert_eq!(tuple3.y, 1.0);
        assert_eq!(tuple3.z, 6.0);
        assert_eq!(tuple3.w, 1.0);
    }

    #[test]
    fn subtract_two_points_returns_vector() {
        let point1 = Tuple::point(3.0, 2.0, 1.0);
        let point2 = Tuple::point(5.0, 6.0, 7.0);
        let vector = point1 - point2;
        assert_eq!(vector.x, -2.0);
        assert_eq!(vector.y, -4.0);
        assert_eq!(vector.z, -6.0);
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    fn subtract_vector_from_point_returns_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);
        let result = point - vector;
        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
        assert_eq!(result.is_point(), true);
    }

    #[test]
    fn subtract_two_vectors_returns_vector() {
        let vector1 = Tuple::vector(3.0, 2.0, 1.0);
        let vector2 = Tuple::vector(5.0, 6.0, 7.0);
        let vector = vector1 - vector2;
        assert_eq!(vector.x, -2.0);
        assert_eq!(vector.y, -4.0);
        assert_eq!(vector.z, -6.0);
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    fn subtract_vector_from_zero_vector_returns_opposite_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let vector = Tuple::vector(1.0, -2.0, 3.0);
        let result = zero - vector;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, -3.0);
        assert_eq!(result.is_vector(), true);
    }

    #[test]
    fn negate_tuple() {
        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let negated = -tuple;
        assert_eq!(negated.x, -1.0);
        assert_eq!(negated.y, 2.0);
        assert_eq!(negated.z, -3.0);
        assert_eq!(negated.w, 4.0);
    }
}
