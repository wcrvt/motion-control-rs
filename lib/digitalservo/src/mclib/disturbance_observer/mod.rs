pub mod native;

use num_traits::Float;
use crate::mclib::integrator;
use crate::algebra::{vector::Vector, matrix::Matrix};

#[derive(Debug, Clone, Copy)]
pub struct VelocityBased <T, const N: usize>
  where [(); N + 1]:
{
  pub kt: T,
  pub jm: T,
  g: Vector<T,  {N + 1}>,
  tu: Vector<T, {N + 1}>,
  ty: Vector<T, {N + 1}>,
  tz: Matrix<T, {N + 1}, {N + 1}>,
  py: Vector<T, {N + 1}>,
  py0_z1: T,
  integrator: [integrator::FirstOrder<T>; N + 1],
}

impl<T: Float + Default + std::fmt::Debug, const N: usize> VelocityBased <T, N> where [(); N + 1]:{

  pub fn new(ts: T, kt: T, jm: T, bandwidth: T) -> Self{

    let pascal_coeff: Vec<T> = pascal_triangle::<T>(N + 1);
    let mut coeff: Vector<T, {N + 1}> = Vector::new();
    for i in 0..(N + 1) {
      coeff[i] = pascal_coeff[i + 1] * bandwidth.powi(i as i32 + 1);
    }
    let g: Vector<T, {N + 1}> = &coeff * jm;

    let d_matrix: Matrix<T, {N + 1}, {N + 1}> = disturbance_matrix::<T, {N + 1}>();
    let mut a_21: Vector<T, {N + 1}> = Vector::new();
    a_21[0] = - T::one() / jm;

    let tu: Vector<T, {N + 1}> = g * (- kt / jm);
    let ty: Vector<T, {N + 1}> = Vector::from([T::zero(); N + 1]);
    let tz: Matrix<T, {N + 1}, {N + 1}> = &d_matrix + &g.outer(&a_21);
    
    Self{
      kt,
      jm,
      g,
      tu,
      ty,
      tz,
      py: Vector::from([T::zero(); {N + 1}]),
      py0_z1: T::zero(),
      integrator: [integrator::FirstOrder::new(ts); {N + 1}],
    }
  }

  pub fn set_kt(mut self, kt: T) -> Self {
    self.kt = kt;
    self.tu = &self.g * (- kt / self.jm);
    self
  }

  pub fn jm(mut self, jm: T) -> Self {

    let d_matrix: Matrix<T, {N + 1}, {N + 1}> = disturbance_matrix::<T, {N + 1}>();
    let mut a_21: Vector<T, {N + 1}> = Vector::new();
    a_21[0] = - T::one() / jm;

    self.jm = jm;
    self.tu = &self.g * (- self.kt / jm);
    self.tz = &d_matrix + &self.g.outer(&a_21);

    self
  }

  pub fn update(&mut self, i: T, v: T) -> T {

    let u: Vector<T, {N + 1}> = &self.tu * i + (&self.tz * &self.g + &self.ty) * v;
    let pi: Vector<T, {N + 1}> = &u + &self.tz * &self.py;

    for i in 0..(N + 1) {
      self.py[i] = self.integrator[i].update(pi[i]);
    }

    let out: T = self.py0_z1 + (&self.g * v)[0];
    self.py0_z1 = self.py[0];
    
    -out
  }
}


fn disturbance_matrix<T: Float + Default, const C: usize>() -> Matrix::<T, C, C> {
  let mut ret: Matrix<T, C, C> = Matrix::<T, C, C>::new();
  for i in 0..C-1 {
    ret[i][i+1] = T::one();
  }
  ret
}

fn pascal_triangle<T: Float + Default> (order: usize) -> Vec<T> {
  let mut m: Vec<Vec<T>> = vec![vec![T::zero(); order + 1]; order + 1];
 
  for i in 1..order + 1 {
    m[0][i] = T::one();
    m[i][0] = T::one();
  }
 
  for i in 1..order + 1 {
    for j in 1..order + 1 - i {
        m[i][j] = m[i][j - 1] + m[i - 1][j]
    }
  }

  let mut ret: Vec<T> = vec![T::zero(); order + 1];
  for i in 0..order + 1 {
    ret[i] = m[order - i][i]
  }

  ret
}