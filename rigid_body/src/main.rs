use std::error::Error;

use digitalservo::plant::rigid_body;
use digitalservo::data_storage::DataStorage;
use digitalservo::mclib::disturbance_observer as dob;
use digitalservo::mclib::quaternion_observer;

fn main()-> Result<(), Box<dyn Error>>  {
  
  //Time step configuration
  let mut t: f64 = 0.0;
  const SLOOP_NUM: usize = 20000;
  const PLOOP_NUM: usize = 10;
  const TS: f64 = 100e-6;
  const TP: f64 = TS / PLOOP_NUM as f64;

  //Logging
  const ROW_SIZE_QUATE: usize = 13;
  const ROW_SIZE_OMEGA: usize = 10;
  const DATAILE_SEPARATOR: &str = ",";
  let output_filename_quate: String = format!("data/quaternion.csv");
  let output_filename_omega: String = format!("data/omega.csv");
  let mut data_storage_quate = DataStorage::<f64, _, ROW_SIZE_QUATE, SLOOP_NUM>::new(output_filename_quate, DATAILE_SEPARATOR);
  let mut data_storage_omega = DataStorage::<f64, _, ROW_SIZE_OMEGA, SLOOP_NUM>::new(output_filename_omega, DATAILE_SEPARATOR);

  //rigid body
  let jb: [f64; 3] = [1.0, 2.0, 5.0];
  let mb: [f64; 3] = [1.0, 1.0, 1.0];
  let q_init: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
  let mut plant = rigid_body::RigidBody::new(TP)
    .set_jb(&jb)
    .set_mb(&mb)
    .set_init_quartenion(&q_init);

  //disturbance observer
  let g: f64 = 500.0;
  let mut dob = [
    dob::VelocityBased::<_, 1>::new(TS, 1.0, jb[0], g),
    dob::VelocityBased::<_, 1>::new(TS, 1.0, jb[1], g),
    dob::VelocityBased::<_, 1>::new(TS, 1.0, jb[2], g)
  ];

  //quaternion observer
  let gain_quaternion: f64 = 0.01;
  let gain_bias: f64 = 0.0001;
  let mut quaternion_observer = quaternion_observer::QuaternionObserver::new(TS)
    .set_mb(&mb)
    .set_gain(gain_quaternion, gain_bias)
    .set_init_quartenion(&[1.0, 0.0, 0.0, 0.0]);

  //command generator
  let rotation_axis: [f64; 3] = [1.0, 4.0, 5.0];
  let rotation_axis_norm: f64 = (rotation_axis[0].powi(2) + rotation_axis[1].powi(2) + rotation_axis[2].powi(2)).sqrt();
  let rotation_axis: [f64; 3] = [rotation_axis[0] / rotation_axis_norm, rotation_axis[1] / rotation_axis_norm, rotation_axis[2] / rotation_axis_norm];
  let rotation_freq: f64 = 2.0;
  let wn: f64 = 2.0 * std::f64::consts::PI * rotation_freq;
  let wn_h: f64 = 0.5 * wn;
  let mut phase: f64;
  let mut phase_h: f64;

  //controller gain
  let kp: f64 = 4900.0;
  let kd: f64 = 2.0 * kp.sqrt();

  //controller signal
  let mut q_cmd: [f64; 4];
  let mut dq_cmd: [f64; 4];
  let mut ddx_ref: [f64; 4] = [0.0; 4];
  let mut ddq_ref: [f64; 3];
  let mut tau_dis_est: [f64; 3] = [0.0; 3];
  let mut torque_cmp: [f64; 3] = [0.0; 3];

  //effector
  let mut force: [f64; 3] = [0.0, 0.0, 0.0];
  let mut torque: [f64; 3] = [0.0; 3];

  //response
  let mut q_res: [f64; 4];
  let mut omega_res: [f64; 3];

  //sensor signal
  let mut omega_sense: [f64; 3];
  let mut acc_b_sense: [f64; 3];
  let mut geomag_b_sense: [f64; 3];

  //estimated state (for debug only)
  let mut omega_bias_hat: [f64; 3];

  for _ in 0..SLOOP_NUM {

    //response
    q_res = plant.quaternion.coordinate.data;
    omega_res = plant.rotation.velocity.data;

    //sense
    acc_b_sense = plant.acceleration_b.data;
    geomag_b_sense = plant.geomag_b.data;
    omega_sense = plant.rotation.velocity.data;

    //biasing
    omega_sense[0] += 0.2;
    omega_sense[1] -= 0.15;
    omega_sense[2] += 0.1;
    
    /* quaternion observer */
    let est = quaternion_observer.estimate(&omega_sense, &force, &acc_b_sense, &geomag_b_sense);
    omega_bias_hat = quaternion_observer.omega_bias_hat.data;

    /* disturbance observer */
    for i in 0..rigid_body::JOINTSPACE_DIM {
      tau_dis_est[i] = dob[i].update(torque[i], est.omega[i]);
      torque_cmp[i] = tau_dis_est[i];
    }
    
    //quaternion command
    phase = wn * t;
    phase_h = phase * 0.5;
    q_cmd = [
      phase_h.cos(),
      phase_h.sin() * rotation_axis[0],
      phase_h.sin() * rotation_axis[1],
      phase_h.sin() * rotation_axis[2],
    ];
    dq_cmd = [
      -wn_h.powi(1) * phase_h.sin(),
       wn_h.powi(1) * phase_h.cos() * rotation_axis[0],
       wn_h.powi(1) * phase_h.cos() * rotation_axis[1],
       wn_h.powi(1) * phase_h.cos() * rotation_axis[2],
    ];

    //reference generator
    for i in 0..rigid_body::QUATERNION_DIM {
      ddx_ref[i] = kp * (q_cmd[i] - est.q[i]) + kd * (dq_cmd[i] - est.dq[i]);
    }

    ddq_ref = plant.computed_torque_method(&ddx_ref);

    for i in 0..rigid_body::JOINTSPACE_DIM {
      torque[i] = ddq_ref[i] * jb[i] + torque_cmp[i];
    }

    force = [0.0, 0.0, 0.0];

    //Simulate
    for _ in 0..PLOOP_NUM {
      plant.update(force, torque);
      t += TP;
    }

    //Logging (quaternion)
    data_storage_quate.add([
      t,
      q_cmd[0], q_cmd[1], q_cmd[2], q_cmd[3], 
      q_res[0], q_res[1], q_res[2], q_res[3],
      est.q[0], est.q[1], est.q[2], est.q[3],
    ]);

    //Logging (omega)
    data_storage_omega.add([
      t,
      omega_res[0], omega_res[1], omega_res[2],
      est.omega[0], est.omega[1], est.omega[2],
      omega_bias_hat[0], omega_bias_hat[1], omega_bias_hat[2]
    ]);
  }

  data_storage_quate.write_file()?;
  data_storage_omega.write_file()?;
      
  Ok(())
}
