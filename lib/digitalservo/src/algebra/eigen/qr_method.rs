use num_traits::Float;
use super::*;

struct QRAlgorithm<T, const N: usize> {
    q: Matrix<T, N, N>,
    r: Matrix<T, N, N>
}

impl <T: Float + Default, const N: usize> Eigen<T, N> {
    
    fn gram_schmidt_process(m: &Matrix<T, N, N>) -> QRAlgorithm<T, N> {
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

        QRAlgorithm {q, r}
    }

    pub fn qr_method(m: &Matrix<T, N, N>) -> Self {
        let mut a: Matrix<T, N, N> = m.clone();
        for _ in 0..100 {
            let qr: QRAlgorithm<T, N> = Self::gram_schmidt_process(&a);
            a = qr.r * qr.q;
        }
        
        let mut value: [T; N] = [T::zero(); N];
        for i in 0..N {
            value[i] = a[i][i];
        }

        let mut vector: [[T; N]; N] = [[T::zero(); N]; N];
        let mu: T = T::from(1e-10).unwrap();
        for i in 0..N {
            let p: Matrix<T, N, N> =(m - Matrix::diag(value[i] + mu)).inverse().unwrap();
            let mut y: Vector<T, N> = Vector::<T, N>::new();
            for i in 0..N{
                y[i] = T::one() / T::from(N).unwrap().sqrt();
            }
            for _ in 0..100 {
                let k: Vector<T, N> = p * y;
                y = k / (k.dot(k)).sqrt();
            }
            vector[i] = y.data;
        }

        Self { value, vector }
    }
} 
