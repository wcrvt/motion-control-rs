use num_traits;

#[derive(Debug, Copy, Clone)]
pub struct FirstOrder <T> {
  y_z1: T,
  u_z1: T,
  pub ts: T,
  coef: [T; 1]
}

impl<T: num_traits::Float> FirstOrder <T> {
  pub fn new(ts: T) -> Self{
    Self{
      y_z1: T::zero(),
      u_z1: T::zero(),
      ts,
      coef: [ts / T::from(2.0).unwrap()]
    }
  }

  pub fn update(&mut self, u: T) -> T {
    let out: T = self.coef[0] * (u + self.u_z1) + self.y_z1;
    self.y_z1 = out;
    self.u_z1 = u;
    out
  }
}

#[derive(Debug, Copy, Clone)]
pub struct SecondOrder <T> {
  y_z1: T,
  y_z2: T,
  u_z1: T,
  u_z2: T,
  pub ts: T,
  coef: [T; 1]
}

impl<T: num_traits::Float> SecondOrder <T> {
  pub fn new(ts: T) -> Self{
    Self{
      y_z1: T::zero(),
      y_z2: T::zero(),
      u_z1: T::zero(),
      u_z2: T::zero(),
      ts,
      coef: [ts * ts / T::from(4.0).unwrap()]
    }
  }

  pub fn update(&mut self, u: T) -> T {
    let t2: T = T::from(2.0).unwrap();
    let out: T = self.coef[0] * (u + t2 * self.u_z1 + self.u_z2) + t2 * self.y_z1 - self.y_z2;
    self.y_z2 = self.y_z1;
    self.u_z2 = self.u_z1;    
    self.y_z1 = out;
    self.u_z1 = u;
    out
  }
}