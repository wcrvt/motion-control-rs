use num_traits;
use crate::mclib::integrator;

#[derive(Debug, Copy, Clone)]
pub struct VelocityBased <T> {
  pub ts: T,
  pub kt: T,
  pub jm: T,
  pub g: [T; 3],
  itg: [integrator::FirstOrder<T>; 3],
  py: [T; 3],
  out_z1: T,
}

impl<T: num_traits::Float> VelocityBased <T> {
  pub fn new(ts: T, kt: T, jm: T, bandwidth: T) -> Self{
    Self{
      ts,
      kt,
      jm,
      g: [jm * T::from(3.0).unwrap() * bandwidth.powi(1), jm * T::from(3.0).unwrap() * bandwidth.powi(2), jm * bandwidth.powi(3)],
      itg: [integrator::FirstOrder::new(ts); 3],
      py: [T::zero(); 3],
      out_z1: T::zero(),
    }
  }

  pub fn update(&mut self, i: T, v: T) -> T {
    let tu: [T; 3] = [- self.kt / self.jm * self.g[0], - self.kt / self.jm * self.g[1], - self.kt / self.jm * self.g[2]];
    let tz: [[T; 3]; 3] = [
      [- T::one() / self.jm * self.g[0], T::one(), T::zero()],
      [- T::one() / self.jm * self.g[1], T::zero(), T::one()],
      [- T::one() / self.jm * self.g[2], T::zero(), T::zero()],
    ];

    let u: [T; 3] = [
      tu[0] * i + tz[0][0] * (self.g[0] * v) + tz[0][1] * (self.g[1] * v) + tz[0][2] * (self.g[2] * v),
      tu[1] * i + tz[1][0] * (self.g[0] * v) + tz[1][1] * (self.g[1] * v) + tz[1][2] * (self.g[2] * v),
      tu[2] * i + tz[2][0] * (self.g[0] * v) + tz[2][1] * (self.g[1] * v) + tz[2][2] * (self.g[2] * v),
    ];

    let pi: [T; 3] = [
      u[0] + tz[0][0] * self.py[0] + tz[0][1] * self.py[1] + tz[0][2] * self.py[2],
      u[1] + tz[1][0] * self.py[0] + tz[1][1] * self.py[1] + tz[1][2] * self.py[2],
      u[2] + tz[2][0] * self.py[0] + tz[2][1] * self.py[1] + tz[2][2] * self.py[2],
    ];

    self.py[0] = self.itg[0].update(pi[0]);
    self.py[1] = self.itg[1].update(pi[1]);
    self.py[2] = self.itg[2].update(pi[2]);

    let out: T = -(self.out_z1 + (self.g[0] * v));
    self.out_z1 = self.py[0];

    out
  }
}