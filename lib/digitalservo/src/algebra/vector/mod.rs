mod binary_operation;
mod compound_assignment;
mod indexing;
mod math;

use super::*;
use std::borrow::Borrow;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const ROWS: usize> {
    pub data: [T; ROWS],
}

impl<T: Default + Copy, const ROWS: usize> Vector<T, ROWS> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); ROWS],
        }
    }
}

impl<T: Default + Copy, S: Borrow<[T; ROWS]>, const ROWS: usize> From<S> for Vector<T, ROWS> {
    fn from(data: S) -> Self {
        let data: &[T; ROWS] = data.borrow();
        Self { data: data.clone() }
    }
}

/* products */
impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    pub fn dot<S: Borrow<Self>>(self, other: S) -> T {
        let other = other.borrow();
        let mut ret: T = T::default();
        for i in 0..ROWS {
            ret = ret + self[i] * other[i];
        }
        ret
    }

    pub fn outer<S: Borrow<Self>>(self, other: S) -> Matrix<T, ROWS, ROWS> {
        let other = other.borrow();
        let mut ret: Matrix<T, ROWS, ROWS> = Matrix::<T, ROWS, ROWS>::new();
        for i in 0..ROWS {
            for j in 0..ROWS {
                ret[i][j] = self[i] * other[j];
            }
        }
        ret
    }
}
