use super::*;
use crate::combinatorics::*;
use num_traits::Float;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct LUPMatrix<T, const N: usize> {
    pub l: Matrix<T, N, N>,
    pub u: Matrix<T, N, N>,
    pub p: Matrix<T, N, N>,
}

impl<T: Float + Default, const N: usize> Eigen<T, N> {
    fn pivoting<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<Matrix<T, N, N>>
    where
        [(); permutation(N, N)]:,
    {
        let m = m.borrow();
        let mut p: Matrix<T, N, N> = Matrix::<T, N, N>::new();
        let permutations: [[usize; N]; permutation(N, N)] = generate_permutation_index::<N>();

        let mut iter: usize = 0;
        'outer: loop {
            let pair: [usize; N] = permutations[iter];
            for i in 0..N {
                if m[pair[i]][i] == T::zero() {
                    iter += 1;
                    if iter == permutation(N, N) {
                        return None;
                    }
                    continue 'outer;
                }
            }

            for i in 0..N {
                p[i][pair[i]] = T::one()
            }
            break;
        }

        Some(p)
    }

    pub fn doolittle_decomposition<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<LUPMatrix<T, N>>
    where
        [(); permutation(N, N)]:,
    {
        let mut m: Matrix<T, N, N> = m.borrow().clone();
        let mut l: Matrix<T, N, N> = Matrix::diag(T::one());
        let mut u: Matrix<T, N, N> = Matrix::new();

        let p: Matrix<T, N, N> = match Self::pivoting(m) {
            Some(p) => p,
            None => return None,
        };

        m = p * m;

        //right-looking algorithm
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

        Some(LUPMatrix { l, u, p })
    }

    pub fn crout_decomposition<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<LUPMatrix<T, N>>
    where
        [(); permutation(N, N)]:,
    {
        let mut m: Matrix<T, N, N> = m.borrow().clone();
        let mut l: Matrix<T, N, N> = Matrix::new();
        let mut u: Matrix<T, N, N> = Matrix::diag(T::one());

        let p: Matrix<T, N, N> = match Self::pivoting(m) {
            Some(p) => p,
            None => return None,
        };

        m = p * m;

        //right-looking algorithm
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

        Some(LUPMatrix { l, u, p })
    }
}
