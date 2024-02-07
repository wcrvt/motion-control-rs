use num_traits;

#[derive(Debug)]
pub struct Trajectory<T> {
  pub coordinate: T,
  pub velocity: T,
  pub acceleration: T
}

impl <T: Default> Trajectory<T> {
  pub fn new()-> Self {
    Self {
      coordinate: T::default(),
      velocity: T::default(),
      acceleration: T::default(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct SmoothTrajectory <T> {
  amp: T,
  freq: T,
  t_start: T,
  t_end: T,
}

impl <T: num_traits::Float + Default> SmoothTrajectory <T> {

  pub fn new(amp: T, freq: T, t_start: T) -> Self {
    let t_end: T = t_start + T::one() / (T::from(2.0).unwrap() * freq);
    Self { amp, freq, t_start, t_end }
  }

  pub fn get_trajectory(&self, t: T) -> Trajectory<T> {

    let t_0: T = T::zero();
    let t_1: T = T::one();
    let t_05: T = T::from(0.5).unwrap();
    let t_2: T = T::from(2.0).unwrap();
    let omega: T = T::from(2.0 * std::f64::consts::PI).unwrap() * self.freq;

    let mut ret = Trajectory::new();
    if t < self.t_start {
      ret.coordinate = t_0;
      ret.velocity = t_0;
      ret.acceleration = t_0;
    }
    else if t < self.t_end {
      ret.coordinate = t_05 * self.amp * (t_1 - (omega * (t - self.t_start)).cos());
      ret.velocity = t_05 * self.amp * omega.powf(t_1) * (omega * (t - self.t_start)).sin();
      ret.acceleration = t_05 * self.amp * omega.powf(t_2) * (omega * (t - self.t_start)).cos();
    }
    else {
      ret.coordinate = self.amp;
      ret.velocity = t_0;
      ret.acceleration = t_0;
    }

    ret
  }
  
}