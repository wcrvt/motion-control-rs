use num::complex::Complex;
use num_traits::{Float, Signed};
use super::newton_method;

pub fn exec<T: Float + Signed>(coefficients: &Vec<T>) -> Vec<Complex<T>>{
  let n: usize = coefficients.len() - 1;
  if n == 0 {return vec![]}

  let c1n: T = coefficients[1] / T::from(n).unwrap();

  if n == 0 { return vec![Complex{re: T::zero(), im: T::zero()}] };
  if n == 1 { return vec![Complex{re: -c1n / coefficients[0], im: T::zero()}] };

  /* Horner's rule (synthetic division) */
  let mut c_origin: Vec<T> = coefficients.clone();
  let mut c_horner: Vec<T> = vec![T::zero(); n + 1];

  for i in (2..n+1).rev() {
    c_horner[i] = c_origin[0];
    for j in 1..i+1 {
      c_origin[j-1] = c_horner[i];
      c_horner[i] = c_horner[i] * c1n + c_origin[j];
    }
  }

  c_horner[0] = coefficients[0];
  c_horner[1] = c_origin[1] + c1n;

  /* Aberth method */
  let c_radius: Vec<T> = c_horner
    .iter()
    .enumerate()
    .map(|(i, x)| if i == 0 { T::one() } else { -*x })
    .collect();

  let r_0: T = coefficients
    .iter()
    .enumerate()
    .map(|(i, &x)| num::abs(x / coefficients[0]).powf(T::from(1.0 / ((i as f64) + 1.0)).unwrap()) )
    .reduce(|p, c| if p < c { c } else { p })
    .unwrap();

  let r: Complex<T> = Complex{re: newton_method::exec(&c_radius, r_0), im: T::zero()};

  let mut z: Vec<Complex<T>> = vec![Complex{re: T::zero(), im: T::zero()}; n];

  for i in 0..n {
    let phase_f64: f64 = (2.0 * (i as f64) - 1.5) * std::f64::consts::PI / (n as f64);
    let phase: Complex<T> = Complex{re: T::zero(), im: T::from(phase_f64).unwrap()};
    z[i] = Complex{re: c1n / coefficients[0], im: T::zero()} + r * phase.exp();
  }

  /* Durand-Kerner method */
  const MAX_ITERATION: usize = 10000;
  let mut iteration: usize = 0;

  fn h<T: Float> (c: &Vec<T>, x: Complex<T>) -> Complex<T> {
    let n: usize = c.len() - 1;
    c
      .iter()
      .enumerate()
      .map(|(i, &coef)| Complex{re: coef, im: T::zero()} * x.powf(T::from(n - i).unwrap()))
      .reduce(|p, c| p + c)
      .unwrap()
  }
  
  fn dh<T: Float> (z: &Vec<Complex<T>>, j: usize) -> Complex<T> {
    let z_j = z[j];
    z
      .iter()
      .enumerate()
      .map(|(i, z_i)| if i == j { Complex{re: T::one(), im: T::zero()} } else { z_j - z_i })
      .reduce(|p, c| p * c)
      .unwrap()
  }

  loop {
    let h_sum: T = z
      .iter()
      .map(|z_i| h(&coefficients, *z_i).norm())
      .reduce(|p, c| p + c)
      .unwrap() / T::from(n).unwrap();

    if h_sum < T::from(1e-15).unwrap() || iteration == MAX_ITERATION { break };

    let mut delta: Vec<Complex<T>> = vec![Complex{re: T::zero(), im: T::zero()}; n];
    for i in 0..n { delta[i] = h(&coefficients, z[i]) / dh(&z, i) };
    for i in 0..n { z[i] = z[i] - delta[i] };

    iteration += 1;
  }

  z
}
