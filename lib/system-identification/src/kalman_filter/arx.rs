use std::collections::VecDeque;
use num_traits::{Float, Signed};
use nalgebra::{DVector, DMatrix};

use super::super::transformation;

pub struct KalmanFilter <T: Float>{
  pub u: VecDeque<T>,
  pub x: VecDeque<T>,
  num_zeros: usize,
  num_poles: usize,
  phi: DVector<T>,
  covariance: DMatrix<T>,
  sigma_v: DMatrix<T>,
  sigma_w: T,
  pub parameter: DVector<T>,
}

impl <T> KalmanFilter <T>
  where T: Float + Signed + nalgebra::ComplexField
{
  pub fn new( num_zeros: usize, num_poles: usize, sigma_v: T, sigma_w: T, cov_0: T) -> Self {
    let psize = num_zeros + num_poles + 1;
    let mut u: VecDeque<T> = VecDeque::with_capacity(num_zeros + 1);
    let mut x: VecDeque<T> = VecDeque::with_capacity(num_poles);
    for _ in 0..num_zeros + 1 { u.push_back(T::zero()) };
    for _ in 0..num_poles { x.push_back(T::zero()) };

    Self {
      x,
      u,
      num_zeros,
      num_poles,
      parameter: DVector::from_element(psize, T::zero()),
      phi: DVector::from_element(psize, T::zero()),
      covariance: DMatrix::identity(psize, psize) * cov_0,
      sigma_v: DMatrix::identity(psize, psize) * sigma_v,
      sigma_w,
    }
  }

  pub fn update(&mut self, u: T, x: T, y: T) {

    //FIFO for input u
    self.u.pop_back();
    self.u.push_front(u);
    //FIFO for state x
    self.x.pop_back();
    self.x.push_front(x);

    for i in 0..self.num_poles { self.phi[i] = self.x[i] };
    for i in 0..self.num_zeros + 1 { self.phi[i + self.num_poles] = self.u[i] };

    let y_est: T = (&self.phi.transpose() * &self.parameter)[(0, 0)];
    let y_err: T = y - y_est;

    //Predict step
    self.covariance += &self.sigma_v;
    
    //Update step
    let uncertainty_sense: T = self.sigma_w;
    let uncertainty_predict: T = (&self.phi.transpose() * &self.covariance * &self.phi)[(0, 0)];
    let uncertainty_observe: T = uncertainty_sense + uncertainty_predict;

    let x = &self.covariance * &self.phi;
    self.parameter += (&x * y_err) / uncertainty_observe;
    self.covariance -= (&x * &x.transpose()) / uncertainty_observe;
  }

  pub fn identify(&mut self, ts: T) -> Option<transformation::Coefficient<T>> {
    let params: &Vec<T> = self.parameter.data.as_vec();

    let mut denom_z: Vec<T> = vec![T::one()];
    for i in  0..self.num_poles { denom_z.push(-params[i]) };
    let numer_z: Vec<T> = params[self.num_poles..].to_vec();

    Some(transformation::exec(&numer_z, &denom_z, ts))
  }
}