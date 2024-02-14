use num_traits::Float;
use std::borrow::Borrow;
use super::*;

struct QRMatrix<T, const N: usize> {
    q: Matrix<T, N, N>,
    r: Matrix<T, N, N>
}

impl <T: Float + Default, const N: usize> Eigen<T, N> {
    
    fn gram_schmidt_process<S: Borrow<Matrix<T, N, N>>>(m: S) -> QRMatrix<T, N> {
        let m: &Matrix<T, N, N> = m.borrow();
        let mt: Matrix<T, N, N> = m.transpose();
    
        let mut a: [Vector<T, N>; N] = [Vector::new(); N];
        for i in 0..N {
            a[i] = Vector::from(mt.data[i])
        }
    
        let mut u: [Vector<T, N>; N] = a;
        for i in 0..N {
            for j in 0..i {
                u[i] = u[i] - a[i].projection(u[j]);
            }
        }
    
        let mut un: [[T; N]; N] = [[T::zero(); N]; N];
        for i in 0..N {
            un[i] = u[i].normalize().data;
        }
    
        let q: Matrix<T, N, N> = Matrix::from(un).transpose();
        let r: Matrix<T, N, N> = q.transpose() * m;

        QRMatrix {q, r}
    }

    pub fn qr_method<S: Borrow<Matrix<T, N, N>>>(m: S) -> Self {
        let m: &Matrix<T, N, N> = m.borrow();
        const ITER_EIGEN_VAL: usize = 100;
        const ITER_EIGEN_VEC: usize = 100;

        let mut a: Matrix<T, N, N> = m.clone();
        for _ in 0..ITER_EIGEN_VAL {
            let mu: Matrix<T, N, N> = Matrix::<T, N, N>::diag(a[N-1][N-1]);
            let qr: QRMatrix<T, N> = Self::gram_schmidt_process(a - mu);
            a = qr.r * qr.q + mu;
        }
        let value: [T; N] = a.diag_elements();

        let mut vector: [[T; N]; N] = [[T::zero(); N]; N];
        let mu: T = T::from(1e-10).unwrap();
        for i in 0..N {
            let p: Matrix<T, N, N> =(m - Matrix::diag(value[i] + mu)).inverse().unwrap();
            let mut y: Vector<T, N> = Vector::<T, N>::new();
            for i in 0..N{
                y[i] = T::one() / T::from(N).unwrap().sqrt();
            }
            for _ in 0..ITER_EIGEN_VEC {
                let k: Vector<T, N> = p * y;
                y = k / (k.dot(k)).sqrt();
            }
            vector[i] = y.data;
        }

        Self { value, vector }
    }
} 
