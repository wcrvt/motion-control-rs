use num_traits::Float;
use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use super::vector::*;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
  pub data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS>
  where T: Default + Copy
{
  fn default() -> Self {
    let data: [[T; COLS]; ROWS] = [[T::default(); COLS]; ROWS];
    Matrix {data}
  }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
  where T: Default + Copy
{
  pub fn new() -> Self { Self { data: [[T::default(); COLS]; ROWS] }}
  //pub fn from(data: [[T; COLS]; ROWS]) -> Self { Self { data } }
  pub fn nrow(&self) -> usize { ROWS }
  pub fn ncol(&self) -> usize { COLS }
}

impl <T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS>
  where T: Default + Copy
{
  fn from(data: [[T; COLS]; ROWS]) -> Matrix<T, ROWS, COLS>{
    Matrix{ data }
  }
}

impl <T, const ROWS: usize, const COLS: usize> From<&[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS>
  where T: Default + Copy
{
  fn from(data: &[[T; COLS]; ROWS]) -> Matrix<T, ROWS, COLS>{
    Matrix{ data: data.clone() }
  }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
  where T: Default + Copy + std::ops::Add<Output=T>
{
  pub fn diag(value: T) -> Self {
    let mut x = Matrix::<T, ROWS, COLS>::new();
    let min = std::cmp::min(COLS, ROWS);
    for i in 0..min {
      x[i][i] = value;
    }
    x
  }
}

/* ---------------- */
/* Binary operation */
/* ---------------- */

/* add matrix */
impl<T, S, const ROWS: usize, const COLS: usize> Add<S> for Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Default + Copy, S: Borrow<Matrix<T, ROWS, COLS>>
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
  where T: Add<Output = T> + Default + Copy, S: Borrow<Matrix<T, ROWS, COLS>>
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
  where T: Sub<Output = T> + Default + Copy, S: Borrow<Matrix<T, ROWS, COLS>>
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
  where T: Sub<Output = T> + Default + Copy, S: Borrow<Matrix<T, ROWS, COLS>>
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
impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<Matrix<T, COLS, K>> for Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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

impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<&Matrix<T, COLS, K>> for Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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

impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<Matrix<T, COLS, K>> for &Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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

impl<T, const K: usize, const ROWS: usize, const COLS: usize> Mul<&Matrix<T, COLS, K>> for &Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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
impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Mul<T> for Matrix<T, ROWS, COLS> {
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

impl<T: Mul<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Mul<T> for &Matrix<T, ROWS, COLS> {
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
impl<T: Div<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Div<T> for Matrix<T, ROWS, COLS> {
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

impl<T: Div<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Div<T> for &Matrix<T, ROWS, COLS> {
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

/* ----------------------------- */
/* compound assignment operation */
/* ----------------------------- */

/* add */
impl<T, S, const ROWS: usize, const COLS: usize> AddAssign<S> for Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Default + Copy, S: Borrow<Self>
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
  where T: Sub<Output = T> + Default + Copy, S: Borrow<Self>
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
  where T: Add<Output = T> + Mul<Output = T> + Default + Copy
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
  where T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy
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
  where T: Mul<Output = T> + Default + Copy
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
  where T: Div<Output = T> + Default + Copy
{
  fn div_assign(&mut self, other: T) {
    for i in 0..ROWS {
      for j in 0..COLS {
        self.data[i][j] = self.data[i][j] / other
      }
    }
  }
}

/* ---------------- */
/* Matrix operation */
/* ---------------- */

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Default + Copy
{
  type Output = [T; COLS];

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS>
  where T: Add<Output = T> + Default + Copy
{
  fn index_mut(&mut self, index: usize) -> &mut [T; COLS] {
    &mut self.data[index]
  }
}

/* transpose */
impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
  pub fn transpose(&self) -> Matrix<T, COLS, ROWS>{
    let mut result = Matrix::<T, COLS, ROWS>::default();
    for i in 0..ROWS {
      for j in 0..COLS {
        result.data[j][i] = self.data[i][j];
      }
    }
    result
  }
}

impl<T: Float + Default, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {

  pub fn inverse (&self) -> Option<Matrix<T, ROWS, COLS>> {
    if ROWS != COLS { return None; }

    let mut m1: Matrix<T, ROWS, COLS> = self.clone();
    let mut m2: Matrix<T, ROWS, COLS> = Matrix::new();

    for i in 0..COLS {
      m2[i][i] = T::one();
    }

    for i in 0..COLS {
      let mut max_row_option: usize = 0;
      let mut max_value_option: T = T::zero();
      for j in i..COLS {
        if m1[j][i].abs() > max_value_option {
          max_value_option = m1[j][i];
          max_row_option = j;
        }
      }

      /*Error Handling */
      if max_value_option == T::zero() { return None }

      let m1_buffer: [T; COLS] = m1[i];
      let m2_buffer: [T; COLS] = m2[i];
      m1[i] = m1[max_row_option];
      m2[i] = m2[max_row_option];
      m1[max_row_option] = m1_buffer;
      m2[max_row_option] = m2_buffer;

      let scaler: T = T::one() / m1[i][i];
      for j in 0..COLS {
        m1[i][j] = m1[i][j] * scaler;
        m2[i][j] = m2[i][j] * scaler;
      }

      for j in 0..COLS {
        if i != j {
          let scaler: T = m1[j][i];
          for k in 0..COLS {
            m1[j][k] = m1[j][k] - scaler * m1[i][k];
            m2[j][k] = m2[j][k] - scaler * m2[i][k];
          }
        }
      }    
    }

    Some(m2)
  }

  pub fn inverse_underdetermined(&self) -> Option<Matrix<T, COLS, ROWS>> {
    if ROWS > COLS {return None}
    let mt: Matrix<T, COLS, ROWS> = self.transpose();
    let m_mt = self * &mt;
    let inv_m_mt = if let Some(x) = m_mt.inverse() { x } else { return None };
    Some(&mt * &inv_m_mt)
  }
  
  pub fn inverse_overdetermined(&self) -> Option<Matrix<T, COLS, ROWS>> {
    if ROWS < COLS {return None}
    let mt = self.transpose();
    let m_mt = &mt * self;
    let inv_m_mt = if let Some(x) = m_mt.inverse() { x } else { return None };
    Some(&inv_m_mt * &mt)
  }
  
  pub fn generalized_inverse(&self) -> Option<Matrix<T, COLS, ROWS>> {
    if ROWS < COLS {self.inverse_underdetermined()} else {self.inverse_overdetermined()}
  }

}