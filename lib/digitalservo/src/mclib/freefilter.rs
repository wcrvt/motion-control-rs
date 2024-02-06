use num_traits::Float;
use crate::algebra::{vector::Vector, matrix::Matrix};

#[derive(Debug)]
pub struct FreeFilter<T, const P: usize, const Z: usize> where [(); P - 1]: {
  a: Matrix<T, {P - 1}, {P - 1}>,
  b: Vector<T, {P - 1}>,
  c: Vector<T, {P - 1}>,
  x: Vector<T, {P - 1}>,
  ts: T,
}

impl <T: Float + Default + std::fmt::Debug, const P: usize, const Z: usize> FreeFilter<T, P, Z> where [(); P - 1]: {
  pub fn new(numer: &[T; Z], denom: &[T; P], ts: T) -> Self {
    if Z > P - 1 {panic!("filter setting error: improper system.")}
    if denom[0] == T::zero() {panic!("filter setting error: invalid characteristic polynomial.")}

    let mut a: Matrix<T, {P - 1}, {P - 1}> = Matrix::new();
    let mut b: Vector<T, {P - 1}> = Vector::new();
    let mut c: Vector<T, {P - 1}> = Vector::new();
    let x: Vector<T, {P - 1}> = Vector::new();

    let numer = Vector::from(numer) / denom[0];
    let denom = Vector::from(denom) / denom[0];

    for i in 0..(P - 2) {
      a[i][i + 1] = T::one();
    }

    for i in 0..(P - 1) {
      a[P - 2][(P - 2) - i] = - denom[i + 1];
      c[(P - 2) - i] = if i < Z { numer[i] } else { T::zero() };
    }

    b[P - 2] = T::one();

    Self { a, b, c, x, ts }
  }

  pub fn update(&mut self, u: T) -> T {
    self.x += (&self.a * &self.x + &self.b * u) * self.ts;
    let y: T = self.c.dot(&self.x);
    y
  }
}
