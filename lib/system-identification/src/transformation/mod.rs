use num_traits::{Float, Signed};
use num::complex::Complex;

pub mod dka_method;
pub mod newton_method;
pub mod vieta_formula;

#[derive(Debug)]
pub struct Coefficient<T> {
  pub denom_z: Vec<T>,
  pub numer_z: Vec<T>,
  pub denom_s: Vec<T>,
  pub numer_s: Vec<T>,
  pub poles_z: Vec<Complex<T>>,
  pub zeros_z: Vec<Complex<T>>,
  pub poles_s: Vec<Complex<T>>,
  pub zeros_s: Vec<Complex<T>>,
  pub dc_gain: T,
}

pub fn exec <T>(numer_z: &Vec<T>, denom_z: &Vec<T>, ts: T) -> Coefficient<T>
  where T: Float + Signed + nalgebra::ComplexField
{
  let denom_z: Vec<T> = denom_z.clone();
  let numer_z: Vec<T> = numer_z.clone();

  let poles_z: Vec<Complex<T>> = dka_method::exec(&denom_z);
  let zeros_z: Vec<Complex<T>> = dka_method::exec(&numer_z);
  
  let c_inv_ts: Complex<T> = Complex{re: T::one() / ts, im: T::zero()};
  let poles_s: Vec<Complex<T>> = poles_z.iter().map(|x| c_inv_ts * x.ln()).collect();
  let zeros_s: Vec<Complex<T>> = zeros_z.iter().map(|x| c_inv_ts * x.ln()).collect();

  let denom_s: Vec<T> = vieta_formula::exec(&poles_s).iter().map(|x| x.re).collect();
  let _numer_s: Vec<T> = vieta_formula::exec(&zeros_s).iter().map(|x| x.re).collect();

  let dc_gain: T = numer_z.iter().fold(T::zero(), |a, &b| a + b) / denom_z.iter().fold(T::zero(), |a, &b| a + b);
  let fraction_reducer: T = denom_s.last().unwrap().clone() / _numer_s.last().unwrap().clone() * dc_gain;

  let numer_s: Vec<T> = _numer_s.iter().map(|&x| x * fraction_reducer).collect();

  Coefficient{denom_z, numer_z, numer_s, denom_s, poles_z, zeros_z, poles_s, zeros_s, dc_gain}
}
