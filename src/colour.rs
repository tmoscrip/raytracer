use std::ops::{Add, Mul, Sub};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[wasm_bindgen]
impl Colour {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }
}

// Colour-specific operations
impl Add for Colour {
    type Output = Colour;
    fn add(self, other: Colour) -> Colour {
        Colour {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Colour {
    type Output = Colour;
    fn sub(self, other: Colour) -> Colour {
        Colour {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;
    fn mul(self, scalar: f64) -> Colour {
        Colour {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;
    fn mul(self, other: Colour) -> Colour {
        Colour {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, AbsDiffEq};

    impl PartialEq for Colour {
        fn eq(&self, other: &Self) -> bool {
            self.r == other.r && self.g == other.g && self.b == other.b
        }
    }

    impl AbsDiffEq for Colour {
        type Epsilon = f64;

        fn default_epsilon() -> Self::Epsilon {
            f64::EPSILON
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            f64::abs_diff_eq(&self.r, &other.r, epsilon)
                && f64::abs_diff_eq(&self.g, &other.g, epsilon)
                && f64::abs_diff_eq(&self.b, &other.b, epsilon)
        }
    }

    #[test]
    fn add_colors() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        let result = c1 + c2;
        assert_abs_diff_eq!(result, Colour::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        let result = c1 - c2;
        assert_abs_diff_eq!(result, Colour::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Colour::new(0.2, 0.3, 0.4);
        let result = c * 2.0;
        assert_abs_diff_eq!(result, Colour::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_colors() {
        let c1 = Colour::new(1.0, 0.2, 0.4);
        let c2 = Colour::new(0.9, 1.0, 0.1);
        let result = c1 * c2;
        assert_abs_diff_eq!(result, Colour::new(0.9, 0.2, 0.04));
    }
}
