use super::*;
use std::ops::{Add, Div, Mul, Sub};

/* add Vector */
impl<T, S: Borrow<Vector<T, ROWS>>, const ROWS: usize> Add<S> for Vector<T, ROWS>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn add(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut ret: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] + other.data[i];
        }
        ret
    }
}

impl<T, S: Borrow<Vector<T, ROWS>>, const ROWS: usize> Add<S> for &Vector<T, ROWS>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn add(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut ret: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] + other.data[i];
        }
        ret
    }
}

/* substract Vector */
impl<T, S: Borrow<Vector<T, ROWS>>, const ROWS: usize> Sub<S> for Vector<T, ROWS>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn sub(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut ret: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] - other.data[i];
        }
        ret
    }
}

impl<T, S: Borrow<Vector<T, ROWS>>, const ROWS: usize> Sub<S> for &Vector<T, ROWS>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn sub(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut ret: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] - other.data[i];
        }
        ret
    }
}

/* multiple by vector (dot product) */
impl<T: Add<Output = T> + Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<&Self>
    for Vector<T, ROWS>
{
    type Output = T;

    fn mul(self, other: &Self) -> Self::Output {
        let mut ret: T = T::default();
        for i in 0..ROWS {
            ret = ret + self.data[i] * other.data[i];
        }
        ret
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<Self>
    for Vector<T, ROWS>
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        let mut ret: T = T::default();
        for i in 0..ROWS {
            ret = ret + self.data[i] * other.data[i];
        }
        ret
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<Self>
    for &Vector<T, ROWS>
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        let mut ret: T = T::default();
        for i in 0..ROWS {
            ret = ret + self.data[i] * other.data[i];
        }
        ret
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<&Self>
    for &Vector<T, ROWS>
{
    type Output = T;

    fn mul(self, other: &Self) -> Self::Output {
        let mut ret: T = T::default();
        for i in 0..ROWS {
            ret = ret + self.data[i] * other.data[i];
        }
        ret
    }
}

/* multiple by scalar */
impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<T> for Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn mul(self, other: T) -> Self::Output {
        let mut ret: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] * other;
        }
        ret
    }
}

impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<T> for &Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn mul(self, other: T) -> Self::Output {
        let mut ret: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] * other;
        }
        ret
    }
}

/* divide by scalar */
impl<T: Div<Output = T> + Default + Copy, const ROWS: usize> Div<T> for Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn div(self, other: T) -> Self::Output {
        let mut ret: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] / other;
        }
        ret
    }
}

impl<T: Div<Output = T> + Default + Copy, const ROWS: usize> Div<T> for &Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn div(self, other: T) -> Self::Output {
        let mut ret: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            ret.data[i] = self.data[i] / other;
        }
        ret
    }
}
