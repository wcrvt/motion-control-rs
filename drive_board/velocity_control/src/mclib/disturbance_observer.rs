use num_traits;

#[derive(Debug, Copy, Clone)]
pub struct FirstOrder <T> {
  z_z1: T,
  u_z1: T,
  pub ts: T,
  pub kt: T,
  pub jm: T,
  pub g: T,
  coef: [T; 2]
}

impl<T: num_traits::Float> FirstOrder <T> {
  pub fn new(ts: T, kt: T, jm: T, bandwidth: T) -> Self{
    let t2: T = T::from(2.0).unwrap();
    let divider = t2 + bandwidth * ts;
    Self{
      z_z1: T::zero(),
      u_z1: T::zero(),
      ts,
      kt,
      jm,
      g: bandwidth,
      coef: [
        bandwidth * ts / divider,
        (t2 - bandwidth * ts) / divider,
      ]
    }
  }

  pub fn update(&mut self, i: T, v: T) -> T {
    let u: T = self.kt * i + self.g * self.jm * v;
    let out =  self.coef[0] * (u + self.u_z1) + self.coef[1] * self.z_z1;
    self.z_z1 = out;
    self.u_z1 = u;
    out - self.g * self.jm * v
  }
}