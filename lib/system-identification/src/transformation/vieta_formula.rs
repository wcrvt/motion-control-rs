use num::complex::Complex;
use num_traits::Float;
use itertools::Itertools;

pub fn exec<T: Float + std::fmt::Debug>(roots: &Vec<Complex<T>>) -> Vec<Complex<T>>{

  fn get_ak<T: Float + std::fmt::Debug>(roots: &Vec<Complex<T>>, k: usize) -> Complex<T> {

    if k == 0 { return Complex{re: T::one(), im: T::zero()} };

    let sign: Complex<T> = if k % 2 == 0 { Complex{re: T::one(), im: T::zero()} } else { Complex{re: -T::one(), im: T::zero()} };

    let n: usize = roots.len();
    let pair: Vec<Vec<usize>> = (0..n).combinations(k).collect();

    let coefficient: Complex<T> = pair
      .iter()
      .map(|x| {
        x
          .iter()
          .fold(Complex{re: T::one(), im: T::zero()}, |a, b| a * roots[*b])
      })
      .fold(Complex{re: T::zero(), im: T::zero()}, |a, b| a + b) * sign;

    coefficient
  }

  let n = roots.len();
  let coefficients: Vec<Complex<T>> = vec![T::zero(); n + 1]
    .iter()
    .enumerate()
    .map(|(i, _)| get_ak(roots, i))
    .collect();

  coefficients

}
