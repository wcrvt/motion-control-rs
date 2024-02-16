use num_traits::Float;
use std::borrow::Borrow;
use super::*;

pub const fn factorial(n: usize) -> usize {
    if n == 1 { 1 } else { n * factorial(n - 1)}
}

#[derive(Debug)]
pub struct LUPMatrix<T, const N: usize> {
    pub l: Matrix<T, N, N>,
    pub u: Matrix<T, N, N>,
    pub p: Matrix<T, N, N>,
}

impl <T: Float + Default, const N: usize> Eigen<T, N> {
    
    fn pivoting<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<Matrix<T, N, N>>
    where
        [(); factorial(N)]:
    {
        let m = m.borrow();
        let mut p: Matrix<T, N, N> = Matrix::<T, N, N>::new();
        let permutation: [[usize; N]; factorial(N)] = generate_permutation::<N>();

        let mut iter: usize = 0;
        'outer: loop {
            let pair: [usize; N] = permutation[iter];
            for i in 0..N {
                let k: usize = pair[i];
                
                if m[k][i] == T::zero() {
                    iter += 1;
                    if iter == factorial(N) { return None }
                    continue 'outer;
                }
            }
            
            for i in 0..N {
                let k: usize = pair[i];
                p[i][k] = T::one()
            } 
            break;
        }

        Some(p)
    }

    pub fn doolittle_decomposition<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<LUPMatrix<T, N>>
    where
        [(); factorial(N)]:
    {
        let mut m: Matrix<T, N, N> = m.borrow().clone();
        let mut l: Matrix<T, N, N> = Matrix::diag(T::one());
        let mut u: Matrix<T, N, N> = Matrix::new();

        let p: Matrix<T, N, N> = match Self::pivoting(m) {
            Some(p) => p,
            None => { return None }
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

        Some(LUPMatrix {l, u, p})
    }

    pub fn crout_decomposition<S: Borrow<Matrix<T, N, N>>>(m: S) -> Option<LUPMatrix<T, N>>
    where
        [(); factorial(N)]:
    {

        let mut m: Matrix<T, N, N> = m.borrow().clone();
        let mut l: Matrix<T, N, N> = Matrix::new();
        let mut u: Matrix<T, N, N> = Matrix::diag(T::one());

        let p: Matrix<T, N, N> = match Self::pivoting(m) {
            Some(p) => p,
            None => { return None }
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

        Some(LUPMatrix {l, u, p})
    }

} 


fn generate_permutation<const N: usize> () -> [[usize; N]; factorial(N)]
where
    [(); factorial(N)]:
{

    fn swap<T: Copy + Clone, S: Borrow<[T; N]>, const N: usize>(v: S, a: usize, b: usize) -> [T; N] {
        let mut v: [T; N] = v.borrow().clone();
        let buffer: T = v[a];
        v[a] = v[b];
        v[b] = buffer;
        v
    }

    let mut base: [usize; N] = [0; N];
    for i in 0..N { base[i] = i as usize; }

    let mut cnt: usize = 0;
    let mut ret: [[usize; N]; factorial(N)] = [[0; N]; factorial(N)];
    ret[cnt] = base;
    cnt += 1;

    for n in 0 ..N {
        for _ in 0..cnt {
            for i in (n + 1)..N {
                ret[cnt] = swap(base, n, i);
                cnt += 1;
            }
        }
    }
    ret
}
