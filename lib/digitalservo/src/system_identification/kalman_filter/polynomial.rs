use num_traits::Float;
use crate::algebra::{vector::Vector, matrix::Matrix};

pub struct KalmanFilter <T, const N: usize> where [(); N + 1]: {
  pub parameter: Vector<T, {N + 1}>,
  covariance: Matrix<T, {N + 1}, {N + 1}>,
  sigma_v: Matrix<T, {N + 1}, {N + 1}>,
  sigma_w: T,
}

impl <T: Float + Default, const N: usize> KalmanFilter <T, N> where [(); N + 1]: {
  pub fn new( sigma_v: T, sigma_w: T, cov_0: T) -> Self {
    Self {
      parameter: Vector::new(),
      covariance: Matrix::diag(cov_0),
      sigma_v: Matrix::diag(sigma_v),
      sigma_w,
    }
  }

  pub fn update(&mut self, x: T, y: T) {
    let mut phi: Vector<T, {N + 1}> = Vector::new();
    for i in 0..(N + 1) {
      phi[i] = x.powi((N - i) as i32);
    }

    let y_est: T = phi.dot(&self.parameter);
    let y_err: T = y - y_est;

    //Predict step
    self.covariance += &self.sigma_v;
    
    //Update step
    let uncertainty_sense: T = self.sigma_w;
    let uncertainty_predict: T = phi.dot(&(&self.covariance * &phi));
    let uncertainty_observe: T = uncertainty_sense + uncertainty_predict;

    let x = &self.covariance * phi;
    self.parameter += (&x * y_err) / uncertainty_observe;
    self.covariance -= x.outer(&x) / uncertainty_observe;

    //use forgetting factor
    //self.covariance = (&self.covariance - (&x * &x.transpose()) / uncertainty_observe) / self.sigma_w;

  }
}