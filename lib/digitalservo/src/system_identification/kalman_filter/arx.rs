use crate::algebra::{matrix::Matrix, vector::Vector};
use num_traits::{Float, Signed};

pub struct KalmanFilter<T, const P: usize, const Z: usize>
where
    [(); P + Z + 1]:,
    [(); Z + 1]:,
{
    pub u: Vector<T, { Z + 1 }>,
    pub x: Vector<T, P>,
    pub parameter: Vector<T, { P + Z + 1 }>,
    covariance: Matrix<T, { P + Z + 1 }, { P + Z + 1 }>,
    sigma_v: Matrix<T, { P + Z + 1 }, { P + Z + 1 }>,
    sigma_w: T,
}

impl<T, const P: usize, const Z: usize> KalmanFilter<T, P, Z>
where
    T: Float + Signed + Default,
    [(); P + Z + 1]:,
    [(); Z + 1]:,
{
    pub fn new(sigma_v: T, sigma_w: T, cov_0: T) -> Self {
        Self {
            x: Vector::new(),
            u: Vector::new(),
            parameter: Vector::new(),
            covariance: Matrix::diag(cov_0),
            sigma_v: Matrix::diag(sigma_v),
            sigma_w,
        }
    }

    pub fn update(&mut self, u: T, x: T, y: T) {
        //FIFO for input u
        for i in (1..(Z + 1)).rev() {
            self.u[i] = self.u[i - 1]
        }
        self.u[0] = u;

        //FIFO for state x
        for i in (1..P).rev() {
            self.x[i] = self.x[i - 1]
        }
        self.x[0] = x;

        let mut phi: Vector<T, { P + Z + 1 }> = Vector::new();
        for i in 0..P {
            phi[i] = self.x[i]
        }
        for i in 0..(Z + 1) {
            phi[i + P] = self.u[i]
        }

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
