mod binary_operation;
mod compound_assignment;
mod inverse;
mod indexing;
mod math;

use std::borrow::Borrow;
use super::*;

impl<T, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    fn default() -> Self {
        let data: [[T; COLS]; ROWS] = [[T::default(); COLS]; ROWS];
        Matrix { data }
    }
}

impl<T, S, const ROWS: usize, const COLS: usize> From<S> for Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
    S: Borrow<[[T; COLS]; ROWS]>
{
    fn from(data: S) -> Matrix<T, ROWS, COLS> {
        let data = data.borrow();
        Matrix { data: data.clone() }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        let data: [[T; COLS]; ROWS] = [[T::default(); COLS]; ROWS];
        Self { data }
    }

    pub fn diag(value: T) -> Self {
        let mut x: Matrix<T, ROWS, COLS> = Matrix::<T, ROWS, COLS>::new();
        let min = std::cmp::min(COLS, ROWS);
        for i in 0..min {
            x[i][i] = value;
        }
        x
    }

    pub fn nrow(&self) -> usize {
        ROWS
    }
    pub fn ncol(&self) -> usize {
        COLS
    }
}

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Default + Copy,
{
    pub fn from_diag_elements<S: Borrow<[T; N]>>(diags: S) -> Self {
        let diags = diags.borrow();
        let mut x: Matrix<T, N, N> = Matrix::<T, N, N>::new();
        for i in 0..N {
            x[i][i] = diags[i];
        }
        x
    }
}

/* transpose */
impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Default + Copy
{
    pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut result = Matrix::<T, COLS, ROWS>::default();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}
