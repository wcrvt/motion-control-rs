use num_traits::Float;
use std::borrow::Borrow;
use super::*;

#[derive(Debug)]
pub struct LUMatrix<T, const N: usize> {
    pub l: Matrix<T, N, N>,
    pub u: Matrix<T, N, N>
}

impl <T: Float + Default, const N: usize> Eigen<T, N> {
    
    pub fn doolittle_decomposition<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<LUMatrix<T, N>> {
        let mut m: Matrix<T, N, N> = m.borrow().clone();
        let mut l: Matrix<T, N, N> = Matrix::diag(T::one());
        let mut u: Matrix<T, N, N> = Matrix::new();

        for i in 0..N {
            if m[i][i] == T::zero() {
                return None
            }
        }
        
        for i in 0..(N - 1) {
            u[i][i] = m[i][i];
            for j in (i + 1)..N {
                u[i][j] = m[i][j];
                l[j][i] = m[j][i] / m[i][i];
            }
            for j in (i + 1)..N {
                for k in (i + 1)..N {
                    m[j][k] = m[j][k] - l[j][i] * u[i][k];
                }
            }
        }

        u[N - 1][N - 1] = m[N - 1][N - 1];

        Some(LUMatrix {l, u})
    }

    pub fn crout_decomposition<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<LUMatrix<T, N>> {

        let mut m: Matrix<T, N, N> = m.borrow().clone();
        let mut l: Matrix<T, N, N> = Matrix::new();
        let mut u: Matrix<T, N, N> = Matrix::diag(T::one());

        for i in 0..N {
            if m[i][i] == T::zero() {
                return None
            }
        }

        for i in 0..(N - 1) {
            l[i][i] = m[i][i];
            for j in (i + 1)..N {
                l[j][i] = m[j][i];
                u[i][j] = m[i][j] / m[i][i];
            }
            for j in (i + 1)..N {
                for k in (i + 1)..N {
                    m[j][k] = m[j][k] - l[j][i] * u[i][k];
                }
            }
        }

        l[N - 1][N - 1] = m[N - 1][N - 1];

        Some(LUMatrix {l, u})
    }

} 
