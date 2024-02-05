use std::collections::VecDeque;
use num_traits::{Float, Signed};
use nalgebra::{self, DVector, DMatrix};

use crate::transformation;

//Use vector dataset
pub fn identify<T> (u: &Vec<T>, x: &Vec<T>, y: &Vec<T>, num_zeros: usize, num_poles: usize, ts: T) -> Option<transformation::Coefficient<T>>
  where T: Float + Signed + nalgebra::ComplexField
{
  let u_len: usize = u.len();
  let x_len: usize = x.len();
  let y_len: usize = y.len();
  let n: usize = *[u_len, x_len, y_len].iter().min().unwrap();

  let vec_size: usize = num_zeros + num_poles + 1;
  let mut u_vec: VecDeque<T> = VecDeque::with_capacity(num_zeros + 1);
  let mut x_vec: VecDeque<T> = VecDeque::with_capacity(num_poles);
  for _ in 0..num_zeros + 1 { u_vec.push_back(T::zero()) };
  for _ in 0..num_poles { x_vec.push_back(T::zero()) };

  let mut phi = DVector::from_element(vec_size, T::zero());
  let mut psi_sum =  DVector::from_element(vec_size, T::zero());
  let mut phi_sum = DMatrix::from_element(vec_size, vec_size, T::zero());

  for i in 0..n {
    //FIFO for input u
    u_vec.pop_back();
    u_vec.push_front(u[i]);
    //FIFO for state x
    x_vec.pop_back();
    x_vec.push_front(x[i]);

    for j in 0..num_poles { phi[j] = x_vec[j] };
    for j in 0..num_zeros + 1 { phi[j + num_poles] = u_vec[j] };

    psi_sum += &phi * y[i];
    phi_sum += &phi * &phi.transpose();
  }

  match &phi_sum.try_inverse() {
    Some(res) => {
      let theta = res * &psi_sum;
      let params = theta.data.as_vec().clone();

      let mut denom_z: Vec<T> = vec![T::one()];
      for i in  0..num_poles { denom_z.push(-params[i]) };
      let numer_z = params[num_poles..].to_vec();

      Some(transformation::exec(&numer_z, &denom_z, ts))
    },
    None => {None}
  }
}

//Use sequential data
pub struct DataBuffer<T> {
  pub u: VecDeque<T>,
  pub x: VecDeque<T>,
  num_zeros: usize,
  num_poles: usize,
  phi: DVector<T>,
  psi_sum: DVector<T>,
  phi_sum: DMatrix<T>,
}

impl <T> DataBuffer<T>
  where T: Float + nalgebra::ComplexField + Signed
{
  pub fn new(num_zeros: usize, num_poles: usize) -> Self {

    let vec_size: usize = num_zeros + num_poles + 1;
    let mut u: VecDeque<T> = VecDeque::with_capacity(num_zeros + 1);
    let mut x: VecDeque<T> = VecDeque::with_capacity(num_poles);
    for _ in 0..num_zeros + 1 { u.push_back(T::zero()) };
    for _ in 0..num_poles { x.push_back(T::zero()) };

    Self {
      u,
      x,
      phi: DVector::from_element(vec_size, T::zero()),
      psi_sum: DVector::from_element(vec_size, T::zero()),
      phi_sum: DMatrix::from_element(vec_size, vec_size, T::zero()),
      num_zeros,
      num_poles,
    }
  }

  pub fn add(&mut self, u: T, x: T, y: T) {
    //FIFO for input u
    self.u.pop_back();
    self.u.push_front(u);
    //FIFO for state x
    self.x.pop_back();
    self.x.push_front(x);

    for i in 0..self.num_poles { self.phi[i] = self.x[i] };
    for i in 0..self.num_zeros + 1 { self.phi[i + self.num_poles] = self.u[i] };

    self.psi_sum += &self.phi * y;
    self.phi_sum += &self.phi * &self.phi.transpose();
  }

  pub fn identify(&mut self, ts: T) -> Option<transformation::Coefficient<T>>{
    match self.phi_sum.clone().try_inverse() {
      Some(res) => {
        let theta = res * &self.psi_sum;
        let params = theta.data.as_vec().clone();

        let mut denom_z: Vec<T> = vec![T::one()];
        for i in  0..self.num_poles { denom_z.push(-params[i]) };
        let numer_z = params[self.num_poles..].to_vec();

        Some(transformation::exec(&numer_z, &denom_z, ts))
      },
      None => {None}
    }
  }

}
