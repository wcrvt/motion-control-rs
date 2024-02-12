use crate::algebra::*;
use num_traits::Float;

pub struct KalmanFilter<T, const P: usize> {
    pub parameter: Vector<T, P>,
    covariance: Matrix<T, P, P>,
    sigma_v: Matrix<T, P, P>,
    sigma_w: T,
}

impl<T: Float + Default, const P: usize> KalmanFilter<T, P> {
    pub fn new(sigma_v: T, sigma_w: T, cov_0: T) -> Self {
        Self {
            parameter: Vector::new(),
            covariance: Matrix::diag(cov_0),
            sigma_v: Matrix::diag(sigma_v),
            sigma_w,
        }
    }

    pub fn update(&mut self, phi: &[T; P], y: T) {
        let phi = Vector::from(phi);

        let y_est: T = phi.dot(&self.parameter);
        let y_err: T = y - y_est;

        //Predict step
        self.covariance += &self.sigma_v;

        //Update step
        let uncertainty_sense: T = self.sigma_w;
        let uncertainty_predict: T = phi.dot(&(&self.covariance * &phi));
        let uncertainty_observe: T = uncertainty_sense + uncertainty_predict;

        let x = &self.covariance * &phi;
        self.parameter += (&x * y_err) / uncertainty_observe;
        self.covariance -= x.outer(&x) / uncertainty_observe;
    }
}
