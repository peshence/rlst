//! Useful macros

/// Generate a new matrix with C Layout
#[macro_export]
macro_rules! rlst_mat {
    ($ScalarType:ty, $dim:expr) => {
        $crate::GenericBaseMatrixMut::<
            $ScalarType,
            $crate::VectorContainer<$ScalarType>,
            $crate::Dynamic,
            $crate::Dynamic,
        >::zeros_from_dim($dim.0, $dim.1)
    };
}

#[macro_export]
macro_rules! rlst_rand_mat {
    ($ScalarType:ty, $dim:expr) => {{
        let mut rng = rand::thread_rng();
        let mut mat = $crate::rlst_mat![$ScalarType, $dim];
        mat.fill_from_rand_standard_normal(&mut rng);
        mat
    }};
}

#[macro_export]
macro_rules! rlst_vec {
    ($ScalarType:ty, $len:expr) => {
        $crate::ColumnVectorD::<$ScalarType>::zeros_from_length($len)
    };
    ($ScalarType:ty, $len:expr, ColumnVector) => {
        $crate::ColumnVectorD::<$ScalarType>::zeros_from_length($len)
    };
    ($ScalarType:ty, $len:expr, RowVector) => {
        $crate::RowVectorD::<$ScalarType>::zeros_from_length($len)
    };
}

#[macro_export]
macro_rules! rlst_rand_vec {
    ($ScalarType:ty, $dim:expr) => {
        rlst_rand_vec![$ScalarType, $dim, ColumnVector]
    };
    ($ScalarType:ty, $dim:expr, $orientation:tt) => {{
        let mut rng = rand::thread_rng();
        let mut vec = $crate::rlst_vec![$ScalarType, $dim, $orientation];
        vec.fill_from_rand_standard_normal(&mut rng);
        vec
    }};
}

#[cfg(test)]
mod test {

    #[test]
    fn create_matrix() {
        let dim = (2, 3);
        let mat = rlst_mat![f64, dim];

        assert_eq!(mat.dim(), (2, 3));
    }

    #[test]
    fn create_random_matrix() {
        let dim = (2, 3);
        let mat = rlst_rand_mat![f64, dim];

        assert_eq!(mat.dim(), (2, 3));
    }

    #[test]
    fn create_column_vector() {
        let length = 5;
        let vec = rlst_vec![f64, length];

        assert_eq!(vec.dim(), (5, 1));
    }

    #[test]
    fn create_row_vector() {
        let length = 5;
        let vec = rlst_vec![f64, length, RowVector];

        assert_eq!(vec.dim(), (1, 5));
    }

    #[test]
    fn create_random_vector() {
        let length = 5;
        let vec = rlst_rand_vec![f64, length];

        assert_eq!(vec.dim(), (5, 1));
    }
}
