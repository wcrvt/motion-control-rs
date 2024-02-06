use num_traits::Float;
use crate::algebra::{vector::Vector, matrix::Matrix};

//Use sequential data
pub struct DataBuffer<T, const P: usize> {
  psi_sum: Vector<T, P>,
  phi_sum: Matrix<T, P, P>,
}

impl <T: Float + Default, const P: usize> DataBuffer<T, P> {
  pub fn new() -> Self {
    Self {
      psi_sum: Vector::new(),
      phi_sum: Matrix::new(),
    }
  }

  pub fn add(&mut self, phi: &[T; P], y: T) {
    let phi = Vector::from(phi);
    self.psi_sum += &phi * y;
    self.phi_sum += phi.outer(&phi);
  }

  pub fn identify(&mut self) -> Option<[T; P]>{
    match self.phi_sum.inverse() {
      Some(res) => {
        let theta = &res * &self.psi_sum;
        Some(theta.data)
      },
      None => {None}
    }
  }
}
