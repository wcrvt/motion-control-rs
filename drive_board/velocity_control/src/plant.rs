use num_traits;

#[derive(Debug, Copy, Clone)]
pub struct Plant <T> {
  pub d0x: T,
  pub d1x: T,
  pub d2x: T,
  pub ts: T,
  pub kt: T,
  pub jm: T,
}

impl <T> Plant <T>
  where T: num_traits::Float + std::ops::AddAssign
{
  pub fn new (ts: T, kt: T, jm: T) -> Self {
    Self {
      d0x: T::zero(),
      d1x: T::zero(),
      d2x: T::zero(),
      ts,
      kt,
      jm
    }
  }

  pub fn update(&mut self, f: T) {
    self.d0x += self.d1x * self.ts;
    self.d1x += self.d2x * self.ts;
    self.d2x = f / self.jm;
  }
}