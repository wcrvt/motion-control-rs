use num_traits::Float;
use nalgebra::{DVector, DMatrix};

//Use vector dataset
pub fn identify<T>(phi: &Vec<Vec<T>>, y: &Vec<T>) -> Option<Vec<T>>
  where T: Float + nalgebra::ComplexField
{
  let phi_len = phi.len();
  let y_len = y.len();
  let n = std::cmp::min(phi_len, y_len);

  let vec_size = phi[0].len();
  let mut psi_sum = DVector::from_element(vec_size, T::zero());
  let mut phi_sum = DMatrix::from_element(vec_size, vec_size, T::zero());

  for i in 0..n {
    let phi_i = DVector::from(phi[i].clone());
    psi_sum += &phi_i * y[i];
    phi_sum += &phi_i * &phi_i.transpose();
  }

  match &phi_sum.try_inverse() {
    Some(res) => {
      let theta = res * psi_sum;
      Some(theta.data.as_vec().clone())
    },
    None => {None}
  }
}

//Use sequential data
pub struct DataBuffer<T> {
  phi: DVector<T>,
  psi_sum: DVector<T>,
  phi_sum: DMatrix<T>,
}

impl <T> DataBuffer<T>
  where T: Float + nalgebra::ComplexField
{
  pub fn new(vec_size: usize) -> Self {
    Self {
      phi: DVector::from_element(vec_size, T::zero()),
      psi_sum: DVector::from_element(vec_size, T::zero()),
      phi_sum: DMatrix::from_element(vec_size, vec_size, T::zero()),
    }
  }

  pub fn add(&mut self, phi: &Vec<T>, y: T) {
    self.phi = DVector::from(phi.clone());
    self.psi_sum += &self.phi * y;
    self.phi_sum += &self.phi * &self.phi.transpose();
  }

  pub fn identify(&mut self) -> Option<Vec<T>>{
    match self.phi_sum.clone().try_inverse() {
      Some(res) => {
        let theta = res * &self.psi_sum;
        Some(theta.data.as_vec().clone())
      },
      None => {None}
    }
  }

}
