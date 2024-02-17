use super::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/* add */
impl<T, S, const ROWS: usize> AddAssign<S> for Vector<T, ROWS>
where
    T: Add<Output = T> + Default + Copy,
    S: Borrow<Self>,
{
    fn add_assign(&mut self, other: S) {
        let other = other.borrow();
        for i in 0..ROWS {
            self.data[i] = self.data[i] + other.data[i];
        }
    }
}

/* Substraction */
impl<T, S, const ROWS: usize> SubAssign<S> for Vector<T, ROWS>
where
    T: Sub<Output = T> + Default + Copy,
    S: Borrow<Self>,
{
    fn sub_assign(&mut self, other: S) {
        let other = other.borrow();
        for i in 0..ROWS {
            self.data[i] = self.data[i] - other.data[i];
        }
    }
}

/* multiply by scalar*/
impl<T, const ROWS: usize> MulAssign<T> for Vector<T, ROWS>
where
    T: Mul<Output = T> + Default + Copy,
{
    fn mul_assign(&mut self, other: T) {
        for i in 0..ROWS {
            self.data[i] = self.data[i] * other;
        }
    }
}

/* divide by scalar*/
impl<T, const ROWS: usize> DivAssign<T> for Vector<T, ROWS>
where
    T: Div<Output = T> + Default + Copy,
{
    fn div_assign(&mut self, other: T) {
        for i in 0..ROWS {
            self.data[i] = self.data[i] / other;
        }
    }
}
