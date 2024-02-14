use std::borrow::Borrow;

use crate::algebra::*;
use num_traits::Float;

pub const JOINTSPACE_DIM: usize = 3;
pub const QUATERNION_DIM: usize = 4;

pub const GRAVITY_R: [f64; 3] = [0.0, 0.0, 9.80665];
pub const GEOMAG_R: [f64; 3] = [1.0, 0.0, 0.0];

#[derive(Debug)]
pub struct DirectionCosineMatrix<T> {
    pub forward: Matrix<T, 3, 3>,
    pub backward: Matrix<T, 3, 3>,
}

impl<T: Float + Default> DirectionCosineMatrix<T> {
    pub fn new() -> Self {
        let forward: Matrix<T, 3, 3> = Matrix::<T, 3, 3>::diag(T::one());
        let backward: Matrix<T, 3, 3> = forward.transpose();
        Self { forward, backward }
    }
}

impl<T: Float + Default, S: Borrow<Vector<T, 4>>> From<S> for DirectionCosineMatrix<T> {
    fn from(q: S) -> Self {
        let q = q.borrow();
        let t2: T = T::from(2.0).unwrap();
        let c: T = q[0].powi(2) - (q[1].powi(2) + q[2].powi(2) + q[3].powi(2));
        let forward: Matrix<T, 3, 3> = Matrix::from([
            [
                c + t2 * q[1] * q[1],
                t2 * (q[1] * q[2] - q[0] * q[3]),
                t2 * (q[1] * q[3] + q[0] * q[2]),
            ],
            [
                t2 * (q[1] * q[2] + q[0] * q[3]),
                c + t2 * q[2] * q[2],
                t2 * (q[2] * q[3] - q[0] * q[1]),
            ],
            [
                t2 * (q[1] * q[3] - q[0] * q[2]),
                t2 * (q[2] * q[3] + q[0] * q[1]),
                c + t2 * q[3] * q[3],
            ],
        ]);
        let backward: Matrix<T, 3, 3> = forward.transpose();
        Self { forward, backward }
    }
}

#[derive(Debug)]
pub struct ObserverOutput<T> {
    pub q: [T; 4],
    pub dq: [T; 4],
    pub omega: [T; 3],
}

pub struct QuaternionObserver<T> {
    pub mb: Vector<T, JOINTSPACE_DIM>,
    pub gain_q: T,
    pub gain_b: T,
    pub q_hat: Vector<T, 4>,
    pub dq_hat: Vector<T, 4>,
    pub omega_hat: Vector<T, 3>,
    pub omega_bias_hat: Vector<T, JOINTSPACE_DIM>,
    pub ts: T,
}

impl<T: Float + Default> QuaternionObserver<T> {
    pub fn new(ts: T) -> Self {
        Self {
            mb: Vector::new(),
            gain_q: T::zero(),
            gain_b: T::zero(),
            q_hat: Vector::new(),
            dq_hat: Vector::new(),
            omega_hat: Vector::new(),
            omega_bias_hat: Vector::new(),
            ts,
        }
    }

    #[must_use]
    pub fn set_mb(mut self, param: &[T; 3]) -> Self {
        self.mb = Vector::from(param);
        self
    }

    #[must_use]
    pub fn set_gain(mut self, gain_q: T, gain_b: T) -> Self {
        self.gain_q = gain_q;
        self.gain_b = gain_b;
        self
    }

    #[must_use]
    pub fn set_init_quartenion(mut self, param: &[T; 4]) -> Self {
        self.q_hat = Vector::from(param);
        self
    }

    fn get_prediction_jacobian(&mut self) -> Matrix<T, 4, 3> {
        let q_h: Vector<T, 4> = self.q_hat * T::from(0.5).unwrap();
        Matrix::from([
            [-q_h[1], -q_h[2], -q_h[3]],
            [q_h[0], -q_h[3], q_h[2]],
            [q_h[3], q_h[0], -q_h[1]],
            [-q_h[2], q_h[1], q_h[0]],
        ])
    }

    fn get_observation_jacobian(&mut self, q: &Vector<T, 4>) -> Matrix<T, 6, 4> {
        let q_a: Vector<T, 4> = q * T::from(2.0 * GRAVITY_R[2]).unwrap();
        let q_m: Vector<T, 4> = q * T::from(2.0 * GEOMAG_R[0]).unwrap();
        Matrix::from([
            [-q_a[2], q_a[3], -q_a[0], q_a[1]],
            [q_a[1], q_a[0], q_a[3], q_a[2]],
            [q_a[0], -q_a[1], -q_a[2], q_a[3]],
            [q_m[0], q_m[1], -q_m[2], -q_m[3]],
            [-q_m[3], q_m[2], q_m[1], -q_m[0]],
            [q_m[2], q_m[3], q_m[0], q_m[1]],
        ])
    }

    pub fn estimate(
        &mut self,
        omega_sense: &[T; 3],
        force: &[T; 3],
        acc_sense: &[T; 3],
        geomag_sense: &[T; 3],
    ) -> ObserverOutput<T> {
        let gravity_r: Vector<T, 3> = Vector::from([
            T::from(GRAVITY_R[0]).unwrap(),
            T::from(GRAVITY_R[1]).unwrap(),
            T::from(GRAVITY_R[2]).unwrap(),
        ]);
        let geomag_r: Vector<T, 3> = Vector::from([
            T::from(GEOMAG_R[0]).unwrap(),
            T::from(GEOMAG_R[1]).unwrap(),
            T::from(GEOMAG_R[2]).unwrap(),
        ]);

        let omega_sense: Vector<T, 3> = Vector::from(omega_sense);

        let force: Vector<T, 3> = Vector::from(force);
        let dyn_acc_b_predict: Vector<T, 3> = Vector::from([
            force[0] / self.mb[0],
            force[1] / self.mb[1],
            force[2] / self.mb[2],
        ]);

        let acc_sense: Vector<T, 3> = Vector::from(acc_sense);
        let geomag_sense: Vector<T, 3> = Vector::from(geomag_sense);
        let y_sense: Vector<T, 6> = Vector::from([
            acc_sense[0],
            acc_sense[1],
            acc_sense[2],
            geomag_sense[0],
            geomag_sense[1],
            geomag_sense[2],
        ]);

        //Preparation
        let jacobian_f: Matrix<T, 4, 3> = self.get_prediction_jacobian();
        let jacobian_fdt: Matrix<T, 4, 3> = jacobian_f * self.ts;
        let inv_jacobian_fdt: Matrix<T, 3, 4> = jacobian_fdt.inverse_overdetermined().unwrap();

        //Observer: prediction (quaternion)
        self.omega_hat = omega_sense + self.omega_bias_hat;
        self.dq_hat = jacobian_f * self.omega_hat;
        let q_predict: Vector<T, 4> = self.q_hat + self.dq_hat * self.ts;

        //Observer: prediction (acceleration_b)
        let dcm: DirectionCosineMatrix<T> = DirectionCosineMatrix::from(q_predict);
        let hx: Matrix<T, 3, 3> = dcm.backward;
        let jacobian_hx: Matrix<T, 6, 4> = self.get_observation_jacobian(&q_predict);
        let inv_jacobian_hx: Matrix<T, 4, 6> = jacobian_hx.inverse_overdetermined().unwrap();
        let acc_b_est: Vector<T, 3> = hx * gravity_r + dyn_acc_b_predict;
        let geomag_b_est: Vector<T, 3> = hx * geomag_r;
        let y_est: Vector<T, 6> = Vector::from([
            acc_b_est[0],
            acc_b_est[1],
            acc_b_est[2],
            geomag_b_est[0],
            geomag_b_est[1],
            geomag_b_est[2],
        ]);

        //Observer: feedback
        let y_err: Vector<T, 6> = y_sense - y_est;
        let delta_q_est: Vector<T, 4> = inv_jacobian_hx * y_err;
        let omega_bias_est: Vector<T, 3> = inv_jacobian_fdt * delta_q_est;

        //Update
        self.q_hat = q_predict + delta_q_est * self.gain_q;
        self.omega_bias_hat += omega_bias_est * self.gain_b;

        let q_hat_norm: T = (self.q_hat[0].powi(2)
            + self.q_hat[1].powi(2)
            + self.q_hat[2].powi(2)
            + self.q_hat[3].powi(2))
        .sqrt();
        self.q_hat /= q_hat_norm;

        ObserverOutput {
            q: self.q_hat.data,
            dq: self.dq_hat.data,
            omega: self.omega_hat.data,
        }
    }
}
