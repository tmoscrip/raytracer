use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn from_vec(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Matrix { data, rows, cols }
    }

    pub fn identity() -> Self {
        let mut matrix = Matrix::new(4, 4);
        matrix.data[0][0] = 1.0;
        matrix.data[1][1] = 1.0;
        matrix.data[2][2] = 1.0;
        matrix.data[3][3] = 1.0;
        matrix
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.data[0][3] = x;
        matrix.data[1][3] = y;
        matrix.data[2][3] = z;
        matrix
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.data[0][0] = x;
        matrix.data[1][1] = y;
        matrix.data[2][2] = z;
        matrix
    }

    pub fn rotation_x(radians: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        let cos_r = radians.cos();
        let sin_r = radians.sin();

        matrix.data[1][1] = cos_r;
        matrix.data[1][2] = -sin_r;
        matrix.data[2][1] = sin_r;
        matrix.data[2][2] = cos_r;

        matrix
    }

    pub fn rotation_y(radians: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        let cos_r = radians.cos();
        let sin_r = radians.sin();

        matrix.data[0][0] = cos_r;
        matrix.data[0][2] = sin_r;
        matrix.data[2][0] = -sin_r;
        matrix.data[2][2] = cos_r;

        matrix
    }

    pub fn rotation_z(radians: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        let cos_r = radians.cos();
        let sin_r = radians.sin();

        matrix.data[0][0] = cos_r;
        matrix.data[0][1] = -sin_r;
        matrix.data[1][0] = sin_r;
        matrix.data[1][1] = cos_r;

        matrix
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.data[0][1] = xy;
        matrix.data[0][2] = xz;
        matrix.data[1][0] = yx;
        matrix.data[1][2] = yz;
        matrix.data[2][0] = zx;
        matrix.data[2][1] = zy;
        matrix
    }

    pub fn transpose(&self) -> Self {
        let mut result = Matrix::new(self.cols, self.rows);

        for row in 0..self.rows {
            for col in 0..self.cols {
                result.data[col][row] = self.data[row][col];
            }
        }

        result
    }

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.cols == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            let mut determinant = 0.0;
            for col in 0..self.cols {
                determinant += self.data[0][col] * self.cofactor(0, col);
            }
            determinant
        }
    }

    // Row to remove
    // Column to remove
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut result = Matrix::new(self.rows - 1, self.cols - 1);

        let mut result_row = 0;
        for matrix_row in 0..self.rows {
            if matrix_row == row {
                continue;
            }

            let mut result_col = 0;
            for matrix_col in 0..self.cols {
                if matrix_col == col {
                    continue;
                }

                result.data[result_row][result_col] = self.data[matrix_row][matrix_col];
                result_col += 1;
            }
            result_row += 1;
        }

        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sub = self.submatrix(row, col);
        return sub.determinant();
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn inverse(&self) -> Matrix {
        let det = self.determinant();
        if det == 0.0 {
            panic!("Matrix is not invertible");
        }

        let mut cofactor_matrix = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                cofactor_matrix[(row, col)] = self.cofactor(row, col);
            }
        }

        let transposed_cofactors = cofactor_matrix.transpose();

        let mut result = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                result[(row, col)] = transposed_cofactors[(row, col)] / det;
            }
        }

        result
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                let diff = (self.data[row][col] - other.data[row][col]).abs();
                if diff > f64::EPSILON {
                    return false;
                }
            }
        }

        true
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, rhs.cols);

        for row in 0..self.rows {
            for col in 0..rhs.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[row][k] * rhs.data[k][col];
                }
                result.data[row][col] = sum;
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let tuple_vec = vec![rhs.x, rhs.y, rhs.z, rhs.w];
        let mut result = vec![0.0; self.rows];

        for row in 0..self.rows {
            let mut sum = 0.0;
            for col in 0..self.cols {
                sum += self.data[row][col] * tuple_vec[col];
            }
            result[row] = sum;
        }

        Tuple::new(result[0], result[1], result[2], result[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, AbsDiffEq};

    impl AbsDiffEq for Matrix {
        type Epsilon = f64;

        fn default_epsilon() -> Self::Epsilon {
            f64::EPSILON
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            if self.rows != other.rows || self.cols != other.cols {
                return false;
            }

            for row in 0..self.rows {
                for col in 0..self.cols {
                    if !f64::abs_diff_eq(&self.data[row][col], &other.data[row][col], epsilon) {
                        return false;
                    }
                }
            }

            true
        }
    }

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let matrix = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 3)], 4.0);
        assert_eq!(matrix[(1, 0)], 5.5);
        assert_eq!(matrix[(1, 2)], 7.5);
        assert_eq!(matrix[(2, 2)], 11.0);
        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[(3, 2)], 15.5);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let matrix = Matrix::from_vec(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);

        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(0, 1)], 5.0);
        assert_eq!(matrix[(1, 0)], 1.0);
        assert_eq!(matrix[(1, 1)], -2.0);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let matrix = Matrix::from_vec(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);

        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(1, 1)], -2.0);
        assert_eq!(matrix[(2, 2)], 1.0);
    }

    #[test]
    fn identical_matrices_are_equal() {
        let matrix_a = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_b = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(matrix_a, matrix_b);
    }

    #[test]
    fn different_matrices_are_not_equal() {
        let matrix_a = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_b = Matrix::from_vec(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(matrix_a, matrix_b);
    }

    #[test]
    fn matrix_can_be_multiplied_by_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_b = Matrix::from_vec(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix::from_vec(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(matrix_a * matrix_b, expected);
    }

    #[test]
    fn matrix_can_be_multiplied_by_tuple() {
        let matrix_a = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let tuple_b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(matrix_a * tuple_b, expected);
    }

    #[test]
    fn matrix_multiplied_by_identity_matrix_equals_itself() {
        let matrix_a = Matrix::from_vec(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);

        let identity = Matrix::identity();

        assert_eq!(matrix_a.clone() * identity, matrix_a);
    }

    #[test]
    fn identity_matrix_multiplied_by_tuple_equals_tuple() {
        let tuple_a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let identity = Matrix::identity();

        assert_eq!(identity * tuple_a, tuple_a);
    }

    #[test]
    fn matrix_can_be_transposed() {
        let matrix_a = Matrix::from_vec(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);

        let expected = Matrix::from_vec(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(matrix_a.transpose(), expected);
    }

    #[test]
    fn transposing_identity_matrix_equals_identity_matrix() {
        let identity = Matrix::identity();
        let transposed_identity = identity.transpose();

        assert_eq!(transposed_identity, Matrix::identity());
    }

    #[test]
    fn determinant_of_2x2_matrix() {
        let matrix = Matrix::from_vec(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);

        assert_eq!(matrix.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);

        let expected = Matrix::from_vec(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);

        assert_eq!(matrix_a.submatrix(0, 2), expected);
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);

        let expected = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);

        assert_eq!(matrix_a.submatrix(2, 1), expected);
    }

    #[test]
    fn calculating_minor_of_3x3_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        let submatrix_b = matrix_a.submatrix(1, 0);
        assert_eq!(submatrix_b.determinant(), 25.0);
        assert_eq!(matrix_a.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_cofactor_of_3x3_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        assert_eq!(matrix_a.minor(0, 0), -12.0);
        assert_eq!(matrix_a.cofactor(0, 0), -12.0);
        assert_eq!(matrix_a.minor(1, 0), 25.0);
        assert_eq!(matrix_a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_determinant_of_3x3_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);

        assert_eq!(matrix_a.cofactor(0, 0), 56.0);
        assert_eq!(matrix_a.cofactor(0, 1), 12.0);
        assert_eq!(matrix_a.cofactor(0, 2), -46.0);
        assert_eq!(matrix_a.determinant(), -196.0);
    }

    #[test]
    fn calculating_determinant_of_4x4_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(matrix_a.cofactor(0, 0), 690.0);
        assert_eq!(matrix_a.cofactor(0, 1), 447.0);
        assert_eq!(matrix_a.cofactor(0, 2), 210.0);
        assert_eq!(matrix_a.cofactor(0, 3), 51.0);
        assert_eq!(matrix_a.determinant(), -4071.0);
    }

    #[test]
    fn testing_invertible_matrix_for_invertibility() {
        let matrix_a = Matrix::from_vec(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);

        assert_eq!(matrix_a.determinant(), -2120.0);
    }

    #[test]
    fn testing_noninvertible_matrix_for_invertibility() {
        let matrix_a = Matrix::from_vec(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(matrix_a.determinant(), 0.0);
    }

    #[test]
    fn calculating_inverse_of_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);

        let b = matrix_a.inverse();

        assert_eq!(matrix_a.determinant(), 532.0);
        assert_eq!(matrix_a.cofactor(2, 3), -160.0);
        assert!((b[(3, 2)] - (-160.0 / 532.0)).abs() < f64::EPSILON);
        assert_eq!(matrix_a.cofactor(3, 2), 105.0);
        assert!((b[(2, 3)] - (105.0 / 532.0)).abs() < f64::EPSILON);

        let expected = Matrix::from_vec(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_abs_diff_eq!(b, expected, epsilon = 0.0001);
    }

    #[test]
    fn calculating_inverse_of_another_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);

        let b = matrix_a.inverse();

        let expected = Matrix::from_vec(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_abs_diff_eq!(b, expected, epsilon = 0.0001);
    }

    #[test]
    fn calculating_inverse_of_third_matrix() {
        let matrix_a = Matrix::from_vec(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);

        let b = matrix_a.inverse();

        let expected = Matrix::from_vec(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_abs_diff_eq!(b, expected, epsilon = 0.0001);
    }

    #[test]
    fn multiplying_product_by_its_inverse() {
        let matrix_a = Matrix::from_vec(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);

        let matrix_b = Matrix::from_vec(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);

        let c = matrix_a.clone() * matrix_b.clone();
        let result = c * matrix_b.inverse();

        assert_abs_diff_eq!(result, matrix_a, epsilon = 0.0001);
    }

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let result = transform * p;
        let expected = Tuple::point(2.0, 1.0, 7.0);

        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn multiplying_by_inverse_of_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let result = inv * p;
        let expected = Tuple::point(-8.0, 7.0, 3.0);

        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        let result = transform * v;

        assert_abs_diff_eq!(result, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        let result = transform * p;
        let expected = Tuple::point(-8.0, 18.0, 32.0);

        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let result = transform * v;
        let expected = Tuple::vector(-8.0, 18.0, 32.0);

        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let result = inv * v;
        let expected = Tuple::vector(-2.0, 2.0, 2.0);

        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Tuple::point(-2.0, 3.0, 4.0);

        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_x(std::f64::consts::PI / 2.0);

        let half_result = half_quarter * p;
        let expected_half = Tuple::point(
            0.0,
            std::f64::consts::SQRT_2 / 2.0,
            std::f64::consts::SQRT_2 / 2.0,
        );
        assert_abs_diff_eq!(half_result, expected_half);

        let full_result = full_quarter * p;
        let expected_full = Tuple::point(0.0, 0.0, 1.0);
        assert_abs_diff_eq!(full_result, expected_full);
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(std::f64::consts::PI / 4.0);
        let inv = half_quarter.inverse();

        let result = inv * p;
        let expected = Tuple::point(
            0.0,
            std::f64::consts::SQRT_2 / 2.0,
            -std::f64::consts::SQRT_2 / 2.0,
        );

        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_y(std::f64::consts::PI / 2.0);

        let half_result = half_quarter * p;
        let expected_half = Tuple::point(
            std::f64::consts::SQRT_2 / 2.0,
            0.0,
            std::f64::consts::SQRT_2 / 2.0,
        );
        assert_abs_diff_eq!(half_result, expected_half);

        let full_result = full_quarter * p;
        let expected_full = Tuple::point(1.0, 0.0, 0.0);
        assert_abs_diff_eq!(full_result, expected_full);
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_z(std::f64::consts::PI / 2.0);

        let half_result = half_quarter * p;
        let expected_half = Tuple::point(
            -std::f64::consts::SQRT_2 / 2.0,
            std::f64::consts::SQRT_2 / 2.0,
            0.0,
        );
        assert_abs_diff_eq!(half_result, expected_half);

        let full_result = full_quarter * p;
        let expected_full = Tuple::point(-1.0, 0.0, 0.0);
        assert_abs_diff_eq!(full_result, expected_full);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Tuple::point(5.0, 3.0, 4.0);
        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Tuple::point(6.0, 3.0, 4.0);
        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Tuple::point(2.0, 5.0, 4.0);
        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Tuple::point(2.0, 7.0, 4.0);
        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Tuple::point(2.0, 3.0, 6.0);
        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Tuple::point(2.0, 3.0, 7.0);
        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(std::f64::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        // apply rotation first
        let p2 = a * p;
        let expected_p2 = Tuple::point(1.0, -1.0, 0.0);
        assert_abs_diff_eq!(p2, expected_p2, epsilon = 0.0001);

        // then apply scaling
        let p3 = b * p2;
        let expected_p3 = Tuple::point(5.0, -5.0, 0.0);
        assert_abs_diff_eq!(p3, expected_p3, epsilon = 0.0001);

        // then apply translation
        let p4 = c * p3;
        let expected_p4 = Tuple::point(15.0, 0.0, 7.0);
        assert_abs_diff_eq!(p4, expected_p4, epsilon = 0.0001);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(std::f64::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let t = c * b * a;
        let result = t * p;
        let expected = Tuple::point(15.0, 0.0, 7.0);
        assert_abs_diff_eq!(result, expected, epsilon = 0.0001);
    }
}
