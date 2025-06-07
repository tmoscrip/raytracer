use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64, // 1 for point, 0 for vector
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

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

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalise(&self) -> Tuple {
        Tuple {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // Only implements for 3D vectors
    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
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

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, scalar: f64) -> Self::Output {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, scalar: f64) -> Self::Output {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, AbsDiffEq};

    impl PartialEq for Tuple {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
        }
    }

    impl AbsDiffEq for Tuple {
        type Epsilon = f64;

        fn default_epsilon() -> Self::Epsilon {
            f64::EPSILON
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            f64::abs_diff_eq(&self.x, &other.x, epsilon)
                && f64::abs_diff_eq(&self.y, &other.y, epsilon)
                && f64::abs_diff_eq(&self.z, &other.z, epsilon)
                && f64::abs_diff_eq(&self.w, &other.w, epsilon)
        }
    }

    #[test]
    fn tuple_point_is_point() {
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(tuple.is_point(), true);
        assert_eq!(tuple.is_vector(), false);
    }

    #[test]
    fn tuple_vector_is_vector() {
        let tuple = Tuple::new(1.0, 2.0, 3.0, 0.0);
        assert_eq!(tuple.is_point(), false);
        assert_eq!(tuple.is_vector(), true);
    }

    #[test]
    fn point_constructor_returns_point() {
        let tuple = Tuple::point(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(tuple, Tuple::new(1.0, 2.0, 3.0, 1.0));
    }

    #[test]
    fn vector_constructor_returns_vector() {
        let tuple = Tuple::vector(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(tuple, Tuple::new(1.0, 2.0, 3.0, 0.0));
    }

    #[test]
    fn add_two_tuples() {
        let tuple1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let tuple3 = tuple1 + tuple2;
        assert_abs_diff_eq!(tuple3, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtract_two_points_returns_vector() {
        let point1 = Tuple::point(3.0, 2.0, 1.0);
        let point2 = Tuple::point(5.0, 6.0, 7.0);
        let vector = point1 - point2;
        assert_abs_diff_eq!(vector, Tuple::new(-2.0, -4.0, -6.0, 0.0));
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    fn subtract_vector_from_point_returns_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);
        let result = point - vector;
        assert_abs_diff_eq!(result, Tuple::new(-2.0, -4.0, -6.0, 1.0));
        assert_eq!(result.is_point(), true);
    }

    #[test]
    fn subtract_two_vectors_returns_vector() {
        let vector1 = Tuple::vector(3.0, 2.0, 1.0);
        let vector2 = Tuple::vector(5.0, 6.0, 7.0);
        let vector = vector1 - vector2;
        assert_abs_diff_eq!(vector, Tuple::new(-2.0, -4.0, -6.0, 0.0));
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    fn subtract_vector_from_zero_vector_returns_opposite_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let vector = Tuple::vector(1.0, -2.0, 3.0);
        let result = zero - vector;
        assert_abs_diff_eq!(result, Tuple::new(-1.0, 2.0, -3.0, 0.0));
        assert_eq!(result.is_vector(), true);
    }

    #[test]
    fn negate_tuple() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let negated = -tuple;
        assert_abs_diff_eq!(negated, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = tuple * 3.5;
        assert_abs_diff_eq!(result, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = tuple * 0.5;
        assert_abs_diff_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = tuple / 2.0;
        assert_abs_diff_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn vector_magnitude() {
        let test_cases = [
            (Tuple::vector(1.0, 0.0, 0.0), 1.0),
            (Tuple::vector(0.0, 1.0, 0.0), 1.0),
            (Tuple::vector(0.0, 0.0, 1.0), 1.0),
            (Tuple::vector(1.0, 2.0, 3.0), 14.0_f64.sqrt()),
            (Tuple::vector(-1.0, -2.0, -3.0), 14.0_f64.sqrt()),
        ];

        for (vector, expected) in test_cases {
            assert_abs_diff_eq!(vector.magnitude(), expected);
        }
    }

    #[test]
    fn vector_normalise() {
        let test_cases = [
            (Tuple::vector(4.0, 0.0, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
            (
                Tuple::vector(1.0, 2.0, 3.0),
                Tuple::vector(0.26726, 0.53452, 0.80178),
            ),
        ];

        for (vector, expected) in test_cases {
            assert_abs_diff_eq!(vector.normalise(), expected, epsilon = 0.0001);
        }
    }

    #[test]
    fn magnitude_of_normalised_vector() {
        let vector = Tuple::vector(1.0, 2.0, 3.0);
        let normalised = vector.normalise();
        assert_eq!(normalised.magnitude(), 1.0)
    }

    #[test]
    fn tuple_dot_product() {
        let vector1 = Tuple::vector(1.0, 2.0, 3.0);
        let vector2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(vector1.dot(&vector2), 20.0);
    }

    #[test]
    fn vector_cross_product() {
        let vector1 = Tuple::vector(1.0, 2.0, 3.0);
        let vector2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_abs_diff_eq!(vector1.cross(&vector2), Tuple::vector(-1.0, 2.0, -1.0));
        assert_abs_diff_eq!(vector2.cross(&vector1), Tuple::vector(1.0, -2.0, 1.0));
    }
}
