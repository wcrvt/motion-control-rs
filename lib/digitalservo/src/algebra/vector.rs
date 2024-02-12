use super::{Matrix, Vector};
use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};


impl<T: Default + Copy, const ROWS: usize> Vector<T, ROWS> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); ROWS],
        }
    }
}

impl<T: Default + Copy, const ROWS: usize> From<[T; ROWS]> for Vector<T, ROWS> {
    fn from(data: [T; ROWS]) -> Self {
        Self { data }
    }
}

impl<T: Default + Copy, const ROWS: usize> From<&[T; ROWS]> for Vector<T, ROWS> {
    fn from(data: &[T; ROWS]) -> Self {
        Self { data: data.clone() }
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize> Index<usize> for Vector<T, ROWS> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize> IndexMut<usize> for Vector<T, ROWS> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}

/* ---------------- */
/* Binary operation */
/* ---------------- */

/* add Vector */
impl<T, S: Borrow<Vector<T, ROWS>>, const ROWS: usize> Add<S> for Vector<T, ROWS>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn add(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut result: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl<T, S: Borrow<Vector<T, ROWS>>, const ROWS: usize> Add<S> for &Vector<T, ROWS>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn add(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut result: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
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
        let mut result: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

impl<T, S: Borrow<Vector<T, ROWS>>, const ROWS: usize> Sub<S> for &Vector<T, ROWS>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn sub(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut result: Vector<T, ROWS> = Vector::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

/* multiple by vector (dot product) */
impl<T: Add<Output = T> + Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<&Self>
    for Vector<T, ROWS>
{
    type Output = T;

    fn mul(self, other: &Self) -> Self::Output {
        let mut result: T = T::default();
        for i in 0..ROWS {
            result = result + self.data[i] * other.data[i];
        }
        result
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<&Self>
    for &Vector<T, ROWS>
{
    type Output = T;

    fn mul(self, other: &Self) -> Self::Output {
        let mut result: T = T::default();
        for i in 0..ROWS {
            result = result + self.data[i] * other.data[i];
        }
        result
    }
}

/* multiple by scalar */
impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<T> for Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn mul(self, other: T) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] * other;
        }
        result
    }
}

impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize> Mul<T> for &Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn mul(self, other: T) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] * other;
        }
        result
    }
}

/* divide by scalar */
impl<T: Div<Output = T> + Default + Copy, const ROWS: usize> Div<T> for Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn div(self, other: T) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] / other;
        }
        result
    }
}

impl<T: Div<Output = T> + Default + Copy, const ROWS: usize> Div<T> for &Vector<T, ROWS> {
    type Output = Vector<T, ROWS>;

    fn div(self, other: T) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            result.data[i] = self.data[i] / other;
        }
        result
    }
}

/* ----------------------------- */
/* compound assignment operation */
/* ----------------------------- */

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

/* products */
impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    pub fn dot(self, other: &Self) -> T {
        let mut ret: T = T::default();
        for i in 0..ROWS {
            ret = ret + self[i] * other[i];
        }
        ret
    }

    pub fn outer(self, other: &Self) -> Matrix<T, ROWS, ROWS> {
        let mut ret: Matrix<T, ROWS, ROWS> = Matrix::<T, ROWS, ROWS>::new();
        for i in 0..ROWS {
            for j in 0..ROWS {
                ret[i][j] = self[i] * other[j];
            }
        }
        ret
    }
}
