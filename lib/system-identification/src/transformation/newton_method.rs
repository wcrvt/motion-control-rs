use num_traits::{Float, Signed};

pub fn exec<T: Float + Signed>(c: &Vec<T>, x0: T) -> T {
  const MAX_ITERATION: usize = 100000;
  let mut iteration: usize = 0;
  let mut x: T = x0;

  fn f<T: Float>(c: &Vec<T>, x: T) -> T {
    let n: usize = c.len() - 1;
    c
      .iter()
      .enumerate()
      .map(|(i, &coef)| coef * x.powf(T::from(n - i).unwrap()))
      .reduce(|p, c| p + c)
      .unwrap()
  }

  fn df<T: Float>(c: &Vec<T>, x: T) -> T {
    let n: usize = c.len() - 1;
    c
      .iter()
      .enumerate()
      .map(|(i, &coef)| if i == n { T::zero() } else { T::from(n - i).unwrap() * coef * x.powf(T::from(n - i - 1).unwrap()) })
      .reduce(|p, c| p + c)
      .unwrap()
  }

  loop {
    let y: T = f(c, x);
    let dy: T = df(c, x);
    if num::abs(y) < T::from(1e-15).unwrap() || iteration == MAX_ITERATION || dy == T::zero() { break };
    x = x - y / dy;
    iteration += 1;
  }

  return x;
}