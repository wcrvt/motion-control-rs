pub struct Eigen<T, const N: usize> {
    pub eigen_value: [T; N],
    pub eigen_vector: [Vector<T, N>; N],
    pub p_matrix: Matrix<T, N, N>
}

impl <T: Float + Default + std::ops::AddAssign + std::fmt::Debug, const N: usize> Eigen<T, N> {

    pub fn givens_rotation(i: usize, j: usize, theta: T) -> Matrix<T, N, N> {
        let mut ret: Matrix<T, N, N> = Matrix::<T, N, N>::diag(T::one());
        let c: T = theta.cos();
        let s: T = theta.sin();
        ret[i][i] = c;
        ret[j][j] = c;
        ret[i][j] = -s;
        ret[j][i] = s;
    
        ret
    }
    
    pub fn search_absolute_maximum(m: &Matrix<T, N, N>) -> [usize; 2]{
        let mut id: [usize; 2] = [0, 1];
        let mut max: T = m[id[0]][id[1]];
        for i in 0..N {
            for j in 0..N {
                if i != j && m[i][j].abs() > max {
                    id = [i, j];
                    max = m[i][j].abs();
                }
            }
        }
        println!("{:?}", max);
        id
    }
    
    pub fn jacobi_eigenvalue_for_symmetric(m: &Matrix<T, N, N>) -> Self {

        let mut diag: Matrix<T, N, N> = m.clone();
        let mut p_matrix: Matrix<T, N, N> = Matrix::<T, N, N>::diag(T::one());
        
        let mut i: usize = 0;
        loop {
            let id: [usize; 2] = Self::search_absolute_maximum(&diag);
            let theta: T = if m[id[0]][id[0]] == m[id[1]][id[1]] {
                T::from(std::f64::consts::PI / 4.0).unwrap()
            } else {
                let y: T = T::from(2.0).unwrap() * m[id[0]][id[1]];
                let x: T = m[id[0]][id[0]] - m[id[1]][id[1]];
                y.atan2(x) * T::from(0.5).unwrap()
            };
            let g: Matrix<T, N, N> = Self::givens_rotation(id[0], id[1], theta);
            diag = g.transpose() * diag * g;
            p_matrix *= g;
    
            let mut nondiag_norm: T = T::zero();
            for i in 0..N {
                for j in 0..N {
                    if i != j {
                        nondiag_norm += diag[i][j].powi(2);
                    }
                }
            }
            if nondiag_norm < T::from(1e-10).unwrap() { break }
            if i > N * N { break }
            i += 1;
        }

        let p_transpose = p_matrix.transpose();
        let mut eigen_value: [T; N] = [T::zero(); N];
        let mut eigen_vector: [Vector<T, N>; N] = [Vector::new(); N];
        for i in 0..N {
            eigen_value[i] = diag[i][i];
            eigen_vector[i] = Vector::from(p_transpose.data[i]);
        }

        Self {
            eigen_value,
            eigen_vector,
            p_matrix,
        }

    }
    
}
