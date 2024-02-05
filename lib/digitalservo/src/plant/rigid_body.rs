use std::borrow::Borrow;

use num_traits::Float;
use crate::algebra::{matrix::Matrix, vector::Vector};

pub const JOINTSPACE_DIM: usize = 3;
pub const QUATERNION_DIM: usize = 4;

pub const GRAVITY_R: [f64; 3] = [0.0, 0.0, 9.80665];
pub const GEOMAG_R: [f64; 3] = [1.0, 0.0, 0.0];

pub fn determinant<T: Float + Default>(m: &Matrix<T, 3, 3>) -> T {
    m[0][0] * m[1][1] * m[2][2]
  + m[0][1] * m[1][2] * m[2][0]
  + m[0][2] * m[1][0] * m[2][1]
  - m[0][0] * m[1][2] * m[2][1]
  - m[0][1] * m[1][0] * m[2][2]
  - m[0][2] * m[1][1] * m[2][0]
}

#[derive(Debug)]
pub struct DirectionCosineMatrix<T> {
  pub forward: Matrix<T, 3, 3>,
  pub backward: Matrix<T, 3, 3>,
}

impl <T: Float + Default> DirectionCosineMatrix<T> {
  pub fn new() -> Self {
    let forward: Matrix<T, 3, 3> = Matrix::<T, 3, 3>::diag(T::one());
    let backward: Matrix<T, 3, 3> = forward.transpose();
    Self { forward, backward }
  }
}

impl <T: Float + Default, S: Borrow<Vector<T, 4>>> From<S> for DirectionCosineMatrix<T> {
  fn from(q: S) -> Self {
    let q = q.borrow();
    let t2: T = T::from(2.0).unwrap();
    let c: T = q[0].powi(2) - (q[1].powi(2) + q[2].powi(2) + q[3].powi(2));
    let forward: Matrix<T, 3, 3> = Matrix::from([
      [ c + t2 * q[1] * q[1], t2 * (q[1] * q[2] - q[0] * q[3]), t2 * (q[1] * q[3] + q[0] * q[2]) ],
      [ t2 * (q[1] * q[2] + q[0] * q[3]), c + t2 * q[2] * q[2], t2 * (q[2] * q[3] - q[0] * q[1]) ],
      [ t2 * (q[1] * q[3] - q[0] * q[2]), t2 * (q[2] * q[3] + q[0] * q[1]), c + t2 * q[3] * q[3] ]
    ]);
    let backward: Matrix<T, 3, 3> = forward.transpose();
    Self { forward, backward }
  }
}

#[derive(Debug)]
pub struct Motion<T, const DIM: usize> {
  pub coordinate: Vector<T, DIM>,
  pub velocity: Vector<T, DIM>,
  pub acceleration: Vector<T, DIM>,
}

impl <T: Float + Default, const DIM: usize> Motion <T, DIM> {
  pub fn new() -> Self {
    Self {
      coordinate: Vector::<T, DIM>::new(),
      velocity: Vector::<T, DIM>::new(),
      acceleration: Vector::<T, DIM>::new(),
    }
  }
}

#[derive(Debug)]
pub struct RigidBody <T: Float> {
  pub translation: Motion<T, JOINTSPACE_DIM>,
  pub rotation: Motion<T, JOINTSPACE_DIM>,
  pub quaternion: Motion<T, QUATERNION_DIM>,
  jacobian: Matrix<T, QUATERNION_DIM, JOINTSPACE_DIM>,
  djacobian: Matrix<T, QUATERNION_DIM, JOINTSPACE_DIM>,
  pub jb: Vector<T, JOINTSPACE_DIM>,
  pub mb: Vector<T, JOINTSPACE_DIM>,
  pub acceleration_b: Vector<T, 3>,
  pub geomag_b: Vector<T, 3>,
  ts: T,
}

impl <T: Float + Default + std::ops::AddAssign + num_traits::Float + std::fmt::Debug> RigidBody<T> {
  
  pub fn new(ts: T) -> Self {
    Self {
      translation: Motion::new(),
      rotation: Motion::new(),
      quaternion: Motion::new(),
      jacobian: Matrix::new(),
      djacobian: Matrix::new(),
      jb: Vector::new(),
      mb: Vector::new(),
      acceleration_b: Vector::new(),
      geomag_b: Vector::new(),
      ts
    }
  }

  #[must_use]
  pub fn set_jb(mut self, param: &[T; 3]) -> Self {
    self.jb = Vector::from(param);
    self
  }

  #[must_use]
  pub fn set_mb(mut self, param: &[T; 3]) -> Self {
    self.mb = Vector::from(param);
    self
  }

  #[must_use]
  pub fn set_init_quartenion(mut self, param: &[T; 4]) -> Self {
    self.quaternion.coordinate = Vector::from(param);
    self.update_system_matrix();
    self
  }

  fn update_system_matrix(&mut self) {
    let q_h: Vector<T, 4> = &self.quaternion.coordinate * T::from(0.5).unwrap();
    let dq_h: Vector<T, 4> = &self.quaternion.velocity * T::from(0.5).unwrap();
    self.jacobian = Matrix::from([
      [-q_h[1], -q_h[2], -q_h[3]],
      [ q_h[0], -q_h[3],  q_h[2]],
      [ q_h[3],  q_h[0], -q_h[1]],
      [-q_h[2],  q_h[1],  q_h[0]]
    ]);
    self.djacobian = Matrix::from([
      [-dq_h[1], -dq_h[2], -dq_h[3]],
      [ dq_h[0], -dq_h[3],  dq_h[2]],
      [ dq_h[3],  dq_h[0], -dq_h[1]],
      [-dq_h[2],  dq_h[1],  dq_h[0]]
    ]);
  }

  pub fn update(&mut self, force: [T; JOINTSPACE_DIM], torque: [T; JOINTSPACE_DIM]) {

    let gravity_r: Vector<T, 3> = Vector::from([T::from(GRAVITY_R[0]).unwrap(), T::from(GRAVITY_R[1]).unwrap(), T::from(GRAVITY_R[2]).unwrap()]);
    let geomag_r: Vector<T, 3> = Vector::from([T::from(GEOMAG_R[0]).unwrap(), T::from(GEOMAG_R[1]).unwrap(), T::from(GEOMAG_R[2]).unwrap()]);

    self.update_system_matrix();

    // Quaternion
    self.quaternion.coordinate += &self.quaternion.velocity * self.ts;
    self.quaternion.velocity = &self.jacobian * &self.rotation.velocity;

    // Prediction
    let dcm: DirectionCosineMatrix<T> = DirectionCosineMatrix::from(&self.quaternion.coordinate);
    let gravity_b: Vector<T, 3> = &dcm.backward * &gravity_r;
    self.geomag_b = &dcm.backward * &geomag_r;

    // Translation of the body
    self.translation.coordinate += &self.translation.velocity * self.ts;
    self.translation.velocity += &self.translation.acceleration * self.ts;

    self.acceleration_b = Vector::from([
      force[0] / self.mb[0] + gravity_b[0],
      force[1] / self.mb[1] + gravity_b[1],
      force[2] / self.mb[2] + gravity_b[2],
    ]);
    self.translation.acceleration = &dcm.forward * &self.acceleration_b;

    // Rotation of the body
    self.rotation.coordinate += &self.rotation.velocity * self.ts;
    self.rotation.velocity += &self.rotation.acceleration * self.ts;
    
    for i in 0..JOINTSPACE_DIM {
      self.rotation.acceleration[i] = torque[i] / self.jb[i];
    }
  }

  pub fn computed_torque_method(&mut self, ddx_ref: &[T; QUATERNION_DIM]) -> [T; JOINTSPACE_DIM] {

    let ddx_ref: Vector<T, 4> = Vector::from(ddx_ref);

    let indexes: [[usize; 3];4] = [[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]];
    let mut i: usize = 0;
    let mut z: usize = 0;
    let mut det_max: T = T::zero();

    for index in indexes {
      let x: Matrix<T, 3, 3> = Matrix::from([self.jacobian[index[0]], self.jacobian[index[1]], self.jacobian[index[2]]]);
      let det: T = determinant(&x).abs();
      if det > det_max {
        det_max = det;
        z = i;
      }
      i += 1;
    }

    let index: [usize; 3] = indexes[z];
    let ddx_ref: Vector<T, 3> = Vector::from([ddx_ref[index[0]], ddx_ref[index[1]], ddx_ref[index[2]]]);
    let jacobian: Matrix<T, 3, 3> = Matrix::from([self.jacobian[index[0]], self.jacobian[index[1]], self.jacobian[index[2]]]);
    let djacobian: Matrix<T, 3, 3> = Matrix::from([self.djacobian[index[0]], self.djacobian[index[1]], self.djacobian[index[2]]]);
    let dq: Vector<T, 3> = Vector::from([self.quaternion.velocity[index[0]], self.quaternion.velocity[index[1]], self.quaternion.velocity[index[2]]]);

    let djaco_dq: Vector<T, 3> = &djacobian * &dq;
    let inv_jacobian: Matrix<T, 3, 3> = jacobian.inverse().unwrap();

    let ddtheta_ref: Vector<T, 3> = &inv_jacobian * (&ddx_ref - &djaco_dq);
    ddtheta_ref.data
  }
}

