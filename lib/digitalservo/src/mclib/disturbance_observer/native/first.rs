use num_traits;
use crate::mclib::integrator;

#[derive(Debug, Copy, Clone)]
pub struct VelocityBased <T> {
  pub ts: T,
  pub kt: T,
  pub jm: T,
  pub g: [T; 2],
  itg: [integrator::FirstOrder<T>; 2],
  py: [T; 2],
  out_z1: T,
}

impl<T: num_traits::Float> VelocityBased <T> {
  pub fn new(ts: T, kt: T, jm: T, bandwidth: T) -> Self{
    Self{
      ts,
      kt,
      jm,
      g: [jm * T::from(2.0).unwrap() * bandwidth, jm * bandwidth * bandwidth],
      itg: [integrator::FirstOrder::new(ts); 2],
      py: [T::zero(); 2],
      out_z1: T::zero(),
    }
  }

  pub fn update(&mut self, i: T, v: T) -> T {
    let tu: [T; 2] = [- self.kt / self.jm * self.g[0],- self.kt / self.jm * self.g[1]];
    let tz: [[T; 2]; 2] = [
      [- T::one() / self.jm * self.g[0], T::one()],
      [- T::one() / self.jm * self.g[1], T::zero()]
    ];

    let u: [T; 2] = [
      tu[0] * i + tz[0][0] * (self.g[0] * v) + tz[0][1] * (self.g[1] * v),
      tu[1] * i + tz[1][0] * (self.g[0] * v) + tz[1][1] * (self.g[1] * v),
    ];

    let pi: [T; 2] = [
      u[0] + tz[0][0] * self.py[0] + tz[0][1] * self.py[1],
      u[1] + tz[1][0] * self.py[0] + tz[1][1] * self.py[1],
    ];

    self.py[0] = self.itg[0].update(pi[0]);
    self.py[1] = self.itg[1].update(pi[1]);

    let out: T = -(self.out_z1 + (self.g[0] * v));
    self.out_z1 = self.py[0];

    out
  }
}