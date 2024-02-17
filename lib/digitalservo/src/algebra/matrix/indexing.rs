use super::*;
use std::ops::{Index, IndexMut};

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut [T; COLS] {
        &mut self.data[index]
    }
}

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Default + Copy,
{
    pub fn row(&self, n: usize) -> [T; N] {
        self[n]
    }

    pub fn row_as_vec(&self, n: usize) -> Vector<T, N> {
        Vector::from(self[n])
    }

    pub fn column(&self, n: usize) -> [T; N] {
        let mut ret: [T; N] = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i][n];
        }
        ret
    }

    pub fn column_as_vec(&self, n: usize) -> Vector<T, N> {
        let mut ret: Vector<T, N> = Vector::<T, N>::new();
        for i in 0..N {
            ret[i] = self[i][n];
        }
        ret
    }

    pub fn diag_elements(&self) -> [T; N] {
        let mut ret: Vector<T, N> = Vector::new();
        for i in 0..N {
            ret[i] = self[i][i];
        }
        ret.data
    }
}
