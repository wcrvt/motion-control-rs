use num_traits::Float;
use super::*;

impl <T: Float + Default, const N: usize> Eigen<T, N> {
    pub fn get_matrix_exponential(&mut self) -> Option<[[T; N]; N]> {
        let p: Matrix<T, N, N> = Matrix::from(self.vector).transpose();
        match p.inverse() {
            Some(p_inv) => {
                let mut t: Matrix<T, N, N> = Matrix::<T, N, N>::new();
                for i in 0..N {
                    t[i][i] = self.value[i].exp();
                }
                let q: Matrix<T, N, N> = p * t * p_inv;
                Some(q.data)
            }
            None => { None }
        }
    }
}