use num_traits::Float;
use nalgebra::{DVector, DMatrix};

pub struct KalmanFilter <T: Float>{
  pub parameter: DVector<T>,
  covariance: DMatrix<T>,
  sigma_v: DMatrix<T>,
  sigma_w: T
}

impl <T> KalmanFilter <T>
  where T: Float + nalgebra::ComplexField
{
  pub fn new( vec_size: usize, sigma_v: T, sigma_w: T, cov_0: T) -> Self {
    Self {
      parameter: DVector::from_element(vec_size, T::zero()),
      covariance: DMatrix::identity(vec_size, vec_size) * cov_0,
      sigma_v: DMatrix::identity(vec_size, vec_size) * sigma_v,
      sigma_w
    }
  }

  pub fn update(&mut self, phi: &Vec<T>, y: T) {
    let phi = DVector::from(phi.clone());
    let y_est: T = (&phi.transpose() * &self.parameter)[(0, 0)];
    let y_err: T = y - y_est;

    //Predict step
    self.covariance += &self.sigma_v;
    
    //Update step
    let uncertainty_sense: T = self.sigma_w;
    let uncertainty_predict: T = (&phi.transpose() * &self.covariance * &phi)[(0, 0)];
    let uncertainty_observe: T = uncertainty_sense + uncertainty_predict;

    let x = &self.covariance * &phi;
    self.parameter += (&x * y_err) / uncertainty_observe;
    self.covariance -= (&x * &x.transpose()) / uncertainty_observe;
  }

  pub fn identify(&mut self) -> Option<Vec<T>> {
    Some(self.parameter.data.as_vec().clone())
  }
}