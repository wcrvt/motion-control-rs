use super::*;
use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/* add */
impl<T, S, const ROWS: usize, const COLS: usize> AddAssign<S> for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Default + Copy,
    S: Borrow<Self>,
{
    fn add_assign(&mut self, other: S) {
        let other = other.borrow();
        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
    }
}

/* substraction */
impl<T, S, const ROWS: usize, const COLS: usize> SubAssign<S> for Matrix<T, ROWS, COLS>
where
    T: Sub<Output = T> + Default + Copy,
    S: Borrow<Self>,
{
    fn sub_assign(&mut self, other: S) {
        let other = other.borrow();
        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
    }
}

/* multiply by matrix*/
impl<T, const ROWS: usize, const COLS: usize> MulAssign<Self> for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        let mut result: Matrix<T, ROWS, COLS> = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                let mut x = T::default();
                for k in 0..ROWS {
                    x = x + self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        self.data = result.data;
    }
}

impl<T, const ROWS: usize, const COLS: usize> MulAssign<&Self> for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy,
{
    fn mul_assign(&mut self, other: &Self) {
        let mut result: Matrix<T, ROWS, COLS> = Self::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                let mut x = T::default();
                for k in 0..ROWS {
                    x = x + self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        self.data = result.data;
    }
}

/* multiply by scalar*/
impl<T, const ROWS: usize, const COLS: usize> MulAssign<T> for Matrix<T, ROWS, COLS>
where
    T: Mul<Output = T> + Default + Copy,
{
    fn mul_assign(&mut self, other: T) {
        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i][j] = self.data[i][j] * other
            }
        }
    }
}

/* divide by scalar*/
impl<T, const ROWS: usize, const COLS: usize> DivAssign<T> for Matrix<T, ROWS, COLS>
where
    T: Div<Output = T> + Default + Copy,
{
    fn div_assign(&mut self, other: T) {
        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i][j] = self.data[i][j] / other
            }
        }
    }
}
