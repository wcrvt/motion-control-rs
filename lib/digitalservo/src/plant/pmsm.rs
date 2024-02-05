use num_traits;
use crate::mclib::integrator;

#[derive(Debug, Copy, Clone)]
pub struct MotionState<T>{
  pub e: T,
  pub m: T
}

impl <T> MotionState<T>
  where T: num_traits::Float
{
  pub fn new() -> Self {
    Self {
      e: T::zero(),
      m: T::zero(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct ThreePhaseState<T>{
  pub u: T,
  pub v: T,
  pub w: T
}

#[derive(Debug, Copy, Clone)]
pub struct DirectQuadrantState<T>{
  pub d: T,
  pub q: T,
  pub z: T,
}

#[derive(Debug, Copy, Clone)]
pub struct AlphaBetaState<T>{
  pub a: T,
  pub b: T,
  pub z: T,
}

impl <T: num_traits::Float> ThreePhaseState <T> {
  pub fn new() -> Self {
    Self{
      u: T::zero(),
      v: T::zero(),
      w: T::zero(),
    }
  }

  pub fn transform_ab(&self) -> AlphaBetaState<T> {
    let t_05 = T::from(0.5).unwrap();
    let c_1: T = (T::from(2.0).unwrap() / T::from(3.0).unwrap()).sqrt();
    let c_2: T = T::one() / T::from(2.0).unwrap().sqrt();
    let c_3: T = T::one() / T::from(3.0).unwrap().sqrt();
    AlphaBetaState {
      a: c_1 * (self.u - t_05 * self.v - t_05 * self.w),
      b: c_2 * (self.v - self.w),
      z: c_3 * (self.u + self.v + self.w),
    }
  }

  pub fn transform_dq(&self, theta: T) -> DirectQuadrantState<T> {
    let c_1: T = (T::from(2.0).unwrap() / T::from(3.0).unwrap()).sqrt();
    let c_2: T = T::one() / T::from(3.0).unwrap().sqrt();
    let phase_offset: T =T::from(2.0).unwrap() / T::from(3.0).unwrap() * T::from(std::f64::consts::PI).unwrap();
    DirectQuadrantState {
      d: c_1 * ((theta).cos() * self.u + (theta - phase_offset).cos() * self.v + (theta + phase_offset).cos() * self.w),
      q: c_1 * (-(theta).sin() * self.u - (theta - phase_offset).sin() * self.v - (theta + phase_offset).sin() * self.w),
      z: c_2 * (self.u + self.v + self. w),
    }
  }
}

impl <T: num_traits::Float> AlphaBetaState <T> {
  pub fn new() -> Self {
    Self{
      a: T::zero(),
      b: T::zero(),
      z: T::zero(),
    }
  }

  pub fn transform_dq(&self, theta: T) -> DirectQuadrantState<T> {
    DirectQuadrantState {
      d: theta.cos() * self.a + theta.sin() * self. b,
      q: -theta.sin() * self.a + theta.cos() * self. b,
      z: self.z,
    }
  }

  pub fn transform_uvw(&self) -> ThreePhaseState<T> {
    let t_05: T = T::from(0.5).unwrap();
    let c_1: T = (T::from(2.0).unwrap() / T::from(3.0).unwrap()).sqrt();
    let c_2: T = T::one() / T::from(2.0).unwrap().sqrt();
    let c_3: T = T::from(3.0).unwrap().sqrt() / T::from(2.0).unwrap();
    ThreePhaseState {
      u: c_1 * (self.a + c_2 * self.z),
      v: c_1 * (-t_05 * self.a + c_3 * self.b + c_2 * self.z),
      w: c_1 * (-t_05 * self.a - c_3 * self.b + c_2 * self.z),
    }
  }
}

impl <T: num_traits::Float> DirectQuadrantState <T> {
  pub fn new() -> Self {
    Self{
      d: T::zero(),
      q: T::zero(),
      z: T::zero(),
    }
  }

  pub fn transform_ab(&self, theta: T) -> AlphaBetaState<T> {
    AlphaBetaState {
      a: theta.cos() * self.d - theta.sin() * self. q,
      b: theta.sin() * self.d + theta.cos() * self. q,
      z: self.z,
    }
  }

  pub fn transform_uvw(&self, theta: T) -> ThreePhaseState<T> {
    let c_1: T = (T::from(2.0).unwrap() / T::from(3.0).unwrap()).sqrt();
    let c_2: T = T::one() / T::from(2.0).unwrap().sqrt();
    let phase_offset: T =T::from(2.0).unwrap() / T::from(3.0).unwrap() * T::from(std::f64::consts::PI).unwrap();
    ThreePhaseState {
      u: c_1 * ((theta).cos() * self.d - (theta).sin() * self.q + c_2 * self.z),
      v: c_1 * ((theta - phase_offset).cos() * self.d - (theta - phase_offset).sin() * self.q + c_2 * self.z),
      w: c_1 * ((theta + phase_offset).cos() * self.d - (theta + phase_offset).sin() * self.q + c_2 * self.z),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct PMSM <T> {
  r_dq: DirectQuadrantState<T>,
  l_dq: DirectQuadrantState<T>,
  np: usize,
  phi_m: T,
  jm: T,
  pub v_uvw: ThreePhaseState<T>,
  pub i_uvw: ThreePhaseState<T>,
  i_dq: DirectQuadrantState<T>,
  pub acc: MotionState<T>,
  pub omega: MotionState<T>,
  pub theta: MotionState<T>,
  pub torque: T,
  integrator_d: integrator::FirstOrder<T>,
  integrator_q: integrator::FirstOrder<T>,
  integrator_omega: integrator::FirstOrder<T>,
  integrator_theta: integrator::FirstOrder<T>,
}

impl <T: num_traits::Float> PMSM <T>
{
  pub fn new(ts: T) -> Self{
    Self {
      r_dq: DirectQuadrantState::new(),
      l_dq: DirectQuadrantState::new(),
      jm: T::zero(),
      v_uvw: ThreePhaseState::new(),
      i_uvw: ThreePhaseState::new(),
      i_dq: DirectQuadrantState::new(),
      acc: MotionState::new(),
      omega: MotionState::new(),
      theta: MotionState::new(),
      np: 1,
      phi_m: T::zero(),
      torque: T::zero(),
      integrator_d: integrator::FirstOrder::new(ts),
      integrator_q: integrator::FirstOrder::new(ts),
      integrator_omega: integrator::FirstOrder::new(ts),
      integrator_theta: integrator::FirstOrder::new(ts),
    }
  }

  #[must_use]
  pub fn set_inductance(mut self, ld: T, lq: T) -> Self{
    self.l_dq.d = ld;
    self.l_dq.q = lq;
    self
  }

  #[must_use]
  pub fn set_resistance(mut self, rd: T, rq: T) -> Self{
    self.r_dq.d = rd;
    self.r_dq.q = rq;
    self
  }

  #[must_use]
  pub fn set_inertia(mut self, jm: T) -> Self{
    self.jm = jm;
    self
  }

  #[must_use]
  pub fn set_phi(mut self, phi_m: T) -> Self{
    self.phi_m = phi_m;
    self
  }

  #[must_use]
  pub fn set_np(mut self, np: usize) -> Self{
    self.np = np;
    self
  }

  pub fn update(&mut self, vin: &ThreePhaseState<T>, tau_dis: T){
    let v_dq: DirectQuadrantState<T> = vin.transform_dq(self.theta.e);
    let mut v_act: DirectQuadrantState<T> = DirectQuadrantState::new();
    v_act.d = v_dq.d + self.omega.e * self.l_dq.q;
    v_act.q = v_dq.q - self.omega.e * self.l_dq.d - self.omega.e * self.phi_m;
    
    self.i_dq.d = self.integrator_d.update((v_act.d - self.r_dq.d * self.i_dq.d) / self.l_dq.d);
    self.i_dq.q = self.integrator_q.update((v_act.q - self.r_dq.q * self.i_dq.q) / self.l_dq.q);
    self.i_uvw = self.i_dq.transform_uvw(self.theta.e);

    let np_t: T = T::from(self.np).unwrap();

    let tau_r: T = np_t * (self.l_dq.d - self.l_dq.q) * self.i_dq.d * self.i_dq.q;
    let tau_m: T = np_t * self.phi_m * self.i_dq.q;
    self.torque = tau_r + tau_m;

    self.theta.m = self.integrator_theta.update(self.omega.m);
    self.omega.m = self.integrator_omega.update(self.acc.m);
    self.acc.m = (self.torque + tau_dis) / self.jm;

    self.theta.e = self.theta.m / np_t;
    self.omega.e = self.omega.m / np_t;
    self.acc.e = self.acc.m / np_t;
  }


}
