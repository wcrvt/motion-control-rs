use std::ops::{AddAssign, MulAssign};

use crate::algebra::*;
use num_traits::Float;

//Use sequential data
pub struct DataBuffer<T, const P: usize, const Z: usize>
where
    [(); P + Z + 1]:,
    [(); Z + 1]:,
{
    pub u: Vector<T, { Z + 1 }>,
    pub x: Vector<T, P>,
    psi_sum: Vector<T, { P + Z + 1 }>,
    phi_sum: Matrix<T, { P + Z + 1 }, { P + Z + 1 }>,
}

impl<T: Float + Default + AddAssign + MulAssign, const P: usize, const Z: usize> DataBuffer<T, P, Z>
where
    [(); P + Z + 1]:,
    [(); Z + 1]:,
{
    pub fn new() -> Self {
        Self {
            x: Vector::new(),
            u: Vector::new(),
            psi_sum: Vector::new(),
            phi_sum: Matrix::new(),
        }
    }

    pub fn add(&mut self, u: T, x: T, y: T) {
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

        self.psi_sum += phi * y;
        self.phi_sum += phi.outer(phi);
    }

    pub fn identify(&mut self) -> Option<[T; P + Z + 1]> {
        match self.phi_sum.inverse() {
            Some(res) => {
                let theta = res * self.psi_sum;
                Some(theta.data)
            }
            None => None,
        }
    }
}
