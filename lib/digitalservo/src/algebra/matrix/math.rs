use super::*;
use num_traits::Float;

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Float + Default,
{
    pub fn trace(&self) -> T {
        let mut ret: T = T::zero();
        for i in 0..N {
            ret = ret + self[i][i];
        }
        ret
    }

    pub fn determinant(&self) -> T {
        let qr_matrix = Eigen::gram_schmidt_process(self);
        let mut ret: T = T::one();
        for i in 0..N {
            ret = ret * qr_matrix.r[i][i];
        }
        ret
    }

    pub fn exp(self) -> Self {
        pub const P: usize = 1000;
        let identity: Matrix<T, N, N> = Matrix::<T, N, N>::diag(T::one());
        let mut ret: Matrix<T, N, N> = identity + self / T::from(P).unwrap();
        for i in (1..P).rev() {
            ret = identity + self / T::from(i).unwrap() * ret;
        }
        ret
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Float + Default,
{
    pub fn frobenius_norm(&self) -> T {
        let mut ret: T = T::zero();
        for i in 0..ROWS {
            for j in 0..COLS {
                ret = ret + self[i][j] * self[i][j];
            }
        }
        ret.sqrt()
    }

    pub fn max_norm(&self) -> T {
        let mut ret = T::zero();
        for i in 0..ROWS {
            for j in 0..COLS {
                if self[i][j].abs() > ret {
                    ret = self[i][j].abs();
                }
            }
        }
        ret
    }
}
