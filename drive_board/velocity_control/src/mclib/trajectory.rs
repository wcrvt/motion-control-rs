use num_traits;

#[derive(Debug, Copy, Clone)]
pub struct SmoothTrajectory <T> {
  amp: T,
  freq: T,
  t_start: T,
  t_end: T,
}

impl <T: num_traits::Float> SmoothTrajectory <T> {
  pub fn new(amp: T, freq: T, t_start: T) -> Self {
    Self {
      amp,
      freq,
      t_start,
      t_end: t_start + T::one() / (T::from(2.0).unwrap() * freq),
    }
  }

  pub fn generate(&self, t: T) -> [T; 3] {

    let t_0: T = T::zero();
    let t_1: T = T::one();
    let t_05: T = T::from(0.5).unwrap();
    let t_2: T = T::from(2.0).unwrap();
    let omega: T = T::from(2.0 * std::f64::consts::PI).unwrap() * self.freq;

    if t < self.t_start {
      [t_0, t_0, t_0]
    }
    else if t < self.t_end {
      [
        t_05 * self.amp * (t_1 - (omega * (t - self.t_start)).cos()),
        t_05 * self.amp * omega.powf(t_1) * (omega * (t - self.t_start)).sin(),
        t_05 * self.amp * omega.powf(t_2) * (omega * (t - self.t_start)).cos()
      ]
    }
    else {
      [self.amp, t_0, t_0]
    }
    
  }
}