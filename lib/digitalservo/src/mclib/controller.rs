use std::ops::AddAssign;

use crate::signal::{differentiator, integrator};
use num_traits;

#[derive(Debug, Copy, Clone)]
pub struct PIController<T> {
    kp: T,
    ki: T,
    err_p: T,
    err_i: T,
    integrator_err: integrator::FirstOrder<T>,
    integrator_limit: Option<T>,
}

impl<T: num_traits::Float> PIController<T> {
    pub fn new(kp: T, ki: T, ts: T) -> Self {
        Self {
            kp,
            ki,
            err_p: T::zero(),
            err_i: T::zero(),
            integrator_err: integrator::FirstOrder::new(ts),
            integrator_limit: None,
        }
    }

    pub fn set_limit(&mut self, limit: T) {
        self.integrator_limit = Some(limit);
    }

    pub fn calc(&mut self, reference: T, response: T) -> T {
        self.err_p = reference - response;

        let is_saturated = if let Some(limit) = self.integrator_limit {
            self.err_i.abs() > limit
        } else {
            false
        };
        if is_saturated == false {
            self.err_i = self.integrator_err.update(self.err_p);
        }

        self.kp * self.err_p + self.ki * self.err_i
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PDController<T> {
    kp: T,
    kd: T,
    err_p: T,
    err_d: T,
    differentiator_err: differentiator::Differentiator<T, 1, 0>,
}

impl<T: num_traits::Float + Default + AddAssign> PDController<T> {
    pub fn new(kp: T, kd: T, g_diff: T, ts: T) -> Self {
        Self {
            kp,
            kd,
            err_p: T::zero(),
            err_d: T::zero(),
            differentiator_err: differentiator::Differentiator::new(ts, g_diff),
        }
    }
    pub fn calc(&mut self, reference: T, response: T) -> T {
        self.err_p = reference - response;
        self.err_d = self.differentiator_err.update(self.err_p);
        self.kp * self.err_p + self.kd * self.err_d
    }
}
