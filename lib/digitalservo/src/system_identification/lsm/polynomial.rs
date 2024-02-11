use crate::algebra::{matrix::Matrix, vector::Vector};
use num_traits::Float;

pub struct DataBuffer<T, const N: usize>
where
    [(); N + 1]:,
{
    psi_sum: Vector<T, { N + 1 }>,
    phi_sum: Matrix<T, { N + 1 }, { N + 1 }>,
}

impl<T: Float + Default, const N: usize> DataBuffer<T, N>
where
    [(); N + 1]:,
{
    pub fn new() -> Self {
        Self {
            psi_sum: Vector::new(),
            phi_sum: Matrix::new(),
        }
    }

    pub fn add(&mut self, x: T, y: T) {
        let mut phi: Vector<T, { N + 1 }> = Vector::new();
        for i in 0..(N + 1) {
            phi[i] = x.powi((N - i) as i32);
        }

        self.psi_sum += &phi * y;
        self.phi_sum += phi.outer(&phi);
    }

    pub fn identify(&mut self) -> Option<[T; N + 1]> {
        match self.phi_sum.inverse() {
            Some(res) => {
                let theta = &res * &self.psi_sum;
                Some(theta.data)
            }
            None => None,
        }
    }
}
