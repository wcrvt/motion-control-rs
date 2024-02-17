use super::*;
use std::borrow::Borrow;
use std::ops::{Add, Div, Mul, Sub};

/* add matrix */
impl<T, S, const ROWS: usize, const COLS: usize> Add<S> for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Default + Copy,
    S: Borrow<Matrix<T, ROWS, COLS>>,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut result: Matrix<T, ROWS, COLS> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

impl<T, S, const ROWS: usize, const COLS: usize> Add<S> for &Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Default + Copy,
    S: Borrow<Matrix<T, ROWS, COLS>>,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut result: Matrix<T, ROWS, COLS> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

/* substract matrix */
impl<T, S, const ROWS: usize, const COLS: usize> Sub<S> for Matrix<T, ROWS, COLS>
where
    T: Sub<Output = T> + Default + Copy,
    S: Borrow<Matrix<T, ROWS, COLS>>,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn sub(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut result: Matrix<T, ROWS, COLS> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
}

impl<T, S, const ROWS: usize, const COLS: usize> Sub<S> for &Matrix<T, ROWS, COLS>
where
    T: Sub<Output = T> + Default + Copy,
    S: Borrow<Matrix<T, ROWS, COLS>>,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn sub(self, other: S) -> Self::Output {
        let other = other.borrow();
        let mut result: Matrix<T, ROWS, COLS> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
}

/* multiple by matrix */
impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<Matrix<T, COLS, K>>
    for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Matrix<T, ROWS, K>;

    fn mul(self, other: Matrix<T, COLS, K>) -> Self::Output {
        let mut result: Matrix<T, ROWS, K> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..K {
                let mut x = T::default();
                for k in 0..COLS {
                    x = x + self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        result
    }
}

impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<&Matrix<T, COLS, K>>
    for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Matrix<T, ROWS, K>;

    fn mul(self, other: &Matrix<T, COLS, K>) -> Self::Output {
        let mut result: Matrix<T, ROWS, K> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..K {
                let mut x = T::default();
                for k in 0..COLS {
                    x = x + self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        result
    }
}

impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<Matrix<T, COLS, K>>
    for &Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Matrix<T, ROWS, K>;

    fn mul(self, other: Matrix<T, COLS, K>) -> Self::Output {
        let mut result: Matrix<T, ROWS, K> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..K {
                let mut x = T::default();
                for k in 0..COLS {
                    x = x + self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        result
    }
}

impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<&Matrix<T, COLS, K>>
    for &Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Matrix<T, ROWS, K>;

    fn mul(self, other: &Matrix<T, COLS, K>) -> Self::Output {
        let mut result: Matrix<T, ROWS, K> = Matrix::new();
        for i in 0..ROWS {
            for j in 0..K {
                let mut x = T::default();
                for k in 0..COLS {
                    x = x + self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        result
    }
}

/* multiple by vector */
impl<T, const ROWS: usize, const COLS: usize> Mul<Vector<T, COLS>> for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn mul(self, other: Vector<T, COLS>) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            let mut x = T::default();
            for j in 0..COLS {
                x = x + self.data[i][j] * other.data[j]
            }
            result.data[i] = x;
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<&Vector<T, COLS>> for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn mul(self, other: &Vector<T, COLS>) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            let mut x = T::default();
            for j in 0..COLS {
                x = x + self.data[i][j] * other.data[j]
            }
            result.data[i] = x;
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<Vector<T, COLS>> for &Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn mul(self, other: Vector<T, COLS>) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            let mut x = T::default();
            for j in 0..COLS {
                x = x + self.data[i][j] * other.data[j]
            }
            result.data[i] = x;
        }
        result
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<&Vector<T, COLS>> for &Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Vector<T, ROWS>;

    fn mul(self, other: &Vector<T, COLS>) -> Self::Output {
        let mut result: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        for i in 0..ROWS {
            let mut x = T::default();
            for j in 0..COLS {
                x = x + self.data[i][j] * other.data[j]
            }
            result.data[i] = x;
        }
        result
    }
}

/* multiple by scalar */
impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Mul<T>
    for Matrix<T, ROWS, COLS>
{
    type Output = Matrix<T, ROWS, COLS>;

    fn mul(self, other: T) -> Self::Output {
        let mut result: Matrix<T, ROWS, COLS> = Matrix::<T, ROWS, COLS>::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] * other;
            }
        }
        result
    }
}

impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Mul<T>
    for &Matrix<T, ROWS, COLS>
{
    type Output = Matrix<T, ROWS, COLS>;

    fn mul(self, other: T) -> Self::Output {
        let mut result: Matrix<T, ROWS, COLS> = Matrix::<T, ROWS, COLS>::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] * other;
            }
        }
        result
    }
}

/* divide by scalar */
impl<T: Div<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Div<T>
    for Matrix<T, ROWS, COLS>
{
    type Output = Matrix<T, ROWS, COLS>;

    fn div(self, other: T) -> Self::Output {
        let mut result: Matrix<T, ROWS, COLS> = Matrix::<T, ROWS, COLS>::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] / other;
            }
        }
        result
    }
}

impl<T: Div<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Div<T>
    for &Matrix<T, ROWS, COLS>
{
    type Output = Matrix<T, ROWS, COLS>;

    fn div(self, other: T) -> Self::Output {
        let mut result: Matrix<T, ROWS, COLS> = Matrix::<T, ROWS, COLS>::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] / other;
            }
        }
        result
    }
}
