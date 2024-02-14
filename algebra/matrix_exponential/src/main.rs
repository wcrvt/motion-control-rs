use num_traits::Float;
use digitalservo::algebra::*;

pub const P: usize = 1000;

fn main() {

    let t = 0.1;
    let m = Matrix::from([[-4.0 * t, t], [0.0, -8.0 * t]]);

    let eigen = QRAlgorithm::search_eigen_value(&m);
    // let q = eigen.get_matrix_exponential();
    // println!("{:.04?}", q);
    println!("{:.04?}", eigen.eigen_vector);

    // let q = get_matrix_exponential(&m);
    // println!("{:.04?}", q.data);

}

pub fn get_matrix_exponential<T: Float + Default, const N: usize>(m: &Matrix<T, N, N>) -> Matrix<T, N, N> {
    let identity: Matrix<T, N, N> = Matrix::<T, N, N>::diag(T::one());
    let mut ret: Matrix<T, N, N>  = identity + m / T::from(P).unwrap();
    for i in (1..P).rev() {
        ret = identity + m / T::from(i).unwrap() * ret;
    }
    ret
}

pub struct Eigen<T, const N: usize> {
    pub eigen_value: [T; N],
    pub eigen_vector: [[T; N]; N]
}

impl <T: Float + Default, const N: usize> Eigen<T, N> {
    pub fn get_matrix_exponential(&mut self) -> [[T; N]; N] {
        let p = Matrix::from(self.eigen_vector).transpose();
        let mut t = Matrix::<T, N, N>::new();
        for i in 0..N {
            t[i][i] = self.eigen_value[i].exp();
        }
        let q: Matrix<T, N, N> = p * t * p.inverse().unwrap();
        q.data
    }
}


pub struct QRAlgorithm<T, const N: usize> {
    q: Matrix<T, N, N>,
    r: Matrix<T, N, N>
}

impl <T: Float + Default + std::fmt::Debug, const N: usize> QRAlgorithm<T, N> {

    
    pub fn qr_decomposition_gram_schmidt(m: &Matrix<T, N, N>) -> Self {
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

        Self {q, r}
    }

    pub fn search_eigen_value(m: &Matrix<T, N, N>) -> Eigen<T, N> {
        let mut a: Matrix<T, N, N> = m.clone();
        for _ in 0..100 {
            let qr: QRAlgorithm<T, N> = Self::qr_decomposition_gram_schmidt(&a);
            a = qr.r * qr.q;
        }
        
        let mut eigen_value: [T; N] = [T::zero(); N];
        for i in 0..N {
            eigen_value[i] = a[i][i];
        }

        let mut eigen_vector: [[T; N]; N] = [[T::zero(); N]; N];
        for i in 0..N {
            let mu: T = T::from(1e-10).unwrap();
            let p: Matrix<T, N, N> =(m - Matrix::diag(eigen_value[i] + mu)).inverse().unwrap();
            let mut y: Vector<T, N> = Vector::<T, N>::new();
            for i in 0..N{
                y[i] = T::one() / T::from(N).unwrap().sqrt();
            }
            for _ in 0..100 {
                let k: Vector<T, N> = p * y;
                y = k / (k.dot(k)).sqrt();
            }
            eigen_vector[i] = y.data;
        }

        Eigen{ eigen_value, eigen_vector }
    }
} 
