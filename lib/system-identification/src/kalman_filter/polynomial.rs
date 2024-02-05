use nalgebra::{DVector, DMatrix};
use num_traits::Float;

pub struct KalmanFilter <T> {
  pub parameter: DVector<T>,
  covariance: DMatrix<T>,
  sigma_v: DMatrix<T>,
  sigma_w: T,
  degree: usize,
}

impl <T> KalmanFilter <T>
  where T: Float + nalgebra::ComplexField
{
  pub fn new( degree: usize, sigma_v: T, sigma_w: T, cov_0: T) -> Self {
    Self {
      parameter: DVector::from_element(degree + 1, T::zero()),
      covariance: DMatrix::identity(degree + 1, degree + 1) * cov_0,
      sigma_v: DMatrix::identity(degree + 1, degree + 1) * sigma_v,
      sigma_w,
      degree,
    }
  }

  pub fn update(&mut self, x: T, y: T) {
    let phi: Vec<T> = vec![0.0; self.degree + 1].iter().enumerate().map(|(i, _)| Float::powi(x, (self.degree - i) as i32)).collect();
    let phi = DVector::from(phi);

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

    //use forgetting factor
    //self.covariance = (&self.covariance - (&x * &x.transpose()) / uncertainty_observe) / self.sigma_w;

  }
}