use crate::{matrix::Matrix, tuple::Tuple};

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    let forward = (to - from).normalise();
    let left = forward.cross(&up.normalise());
    let true_up = left.cross(&forward);

    let orientation = Matrix::from_vec(vec![
        vec![left.x, left.y, left.z, 0.0],
        vec![true_up.x, true_up.y, true_up.z, 0.0],
        vec![-forward.x, -forward.y, -forward.z, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    orientation * Matrix::translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::{matrix::Matrix, tuple::Tuple};

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        let expected = Matrix::from_vec(vec![
            vec![-0.50709, 0.50709, 0.67612, -2.36643],
            vec![0.76772, 0.60609, 0.12122, -2.82843],
            vec![-0.35857, 0.59761, -0.71714, 0.00000],
            vec![0.00000, 0.00000, 0.00000, 1.00000],
        ]);

        assert_abs_diff_eq!(t, expected, epsilon = 0.0001);
    }
}
