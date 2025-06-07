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

    pub fn transpose(&self) -> Self {
        let mut result = Matrix::new(self.cols, self.rows);

        for row in 0..self.rows {
            for col in 0..self.cols {
                result.data[col][row] = self.data[row][col];
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
}
