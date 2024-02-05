use rand_distr::{Normal, Distribution};

use digitalservo::data_storage::DataStorage;
use system_identification::{kalman_filter, lsm};

fn main() {
  //Time step setting
  const TS: f64 = 12.5e-6;
  const TS_DIV: usize = 100;
  const TP: f64 = TS / TS_DIV as f64;
  const DATALEN: usize = 10000;

  let mut t: f64 = 0.0;

  //Noise generator
  let mut rng = rand::thread_rng();
  let noise_v: Normal<f64> = Normal::new(0.0, 0.001).unwrap();
  let noise_w: Normal<f64> = Normal::new(0.0, 0.001).unwrap();
  
  //Parameter model
  let jm: f64 = 28e-6;
  let dm: f64 = 1e-5;

  //State variables
  let mut u: f64;
  let mut omega: f64 = 0.0;
  let mut acc: f64 = 0.0;

  //System identificator (ARX)
  let mut s_arx_lsm = lsm::arx::DataBuffer::<f64>::new(0, 1);
  let mut s_arx_kf = kalman_filter::arx::KalmanFilter::<f64>::new(0, 1, 0.1, 1.0, 10000.0);
  
  //System identificator (Whitebox)
  let mut s_wb_lsm = lsm::whitebox::DataBuffer::<f64>::new(2);
  let mut s_wb_kf = kalman_filter::whitebox::KalmanFilter::<f64>::new(2, 0.1, 1.0, 10000.0);

  //Data storage
  const ROW_SIZE: usize = 2;
  const DATA_SIZE: usize = DATALEN;
  const DATAILE_SEPARATOR: &str = ",";
  let output_filename: String = format!("data/out.csv");
  let mut data_storage = DataStorage::<f64, _, ROW_SIZE, DATA_SIZE>::new(output_filename, DATAILE_SEPARATOR);

  for _ in 0..DATALEN {

    let param: &Vec<f64> = s_wb_kf.parameter.data.as_vec();
    data_storage.add([param[0], param[1]]);

    //Input
    //u = if t > 0.0 {1.0} else {0.0};
    u = 0.0;
    for freq in (100..200).step_by(10) {
      u += 1.0 * (2.0 * std::f64::consts::PI * (freq as f64) * t).sin();
    }

    //Sampler
    let uref_sampled: f64 = u;
    let omega_sampled: f64 = omega;

    // Plant simulator
    for _ in 0..TS_DIV {
      omega += acc * TP;
      acc = ((u + noise_v.sample(&mut rng)) - dm * omega) / jm;
    };

    //Sense
    let omega_sensed: f64 = omega + noise_w.sample(&mut rng);
    let acc_sensed: f64 = acc + noise_w.sample(&mut rng);

    //Identificator
    s_arx_lsm.add(uref_sampled, omega_sampled, omega_sensed);
    s_arx_kf.update(uref_sampled, omega_sampled, omega_sensed);
    s_wb_lsm.add(&vec![acc_sensed, omega_sensed], uref_sampled);
    s_wb_kf.update(&vec![acc_sensed, omega_sensed], uref_sampled);

    t += TS;
  }

  println!("\nSampling time: {TS:?}\n");

  let x = s_arx_lsm.identify(TS).unwrap();
  let params_arx_lsm: [f64; 2] = [x.denom_s[0] / x.numer_s[0], x.denom_s[1] / x.numer_s[0]];

  let x = s_arx_kf.identify(TS).unwrap();
  let params_arx_kf: [f64; 2] = [x.denom_s[0] / x.numer_s[0], x.denom_s[1] / x.numer_s[0]];

  let params_lsm_wb: Vec<f64> = s_wb_lsm.identify().unwrap();
  let params_lsm_kf: Vec<f64> = s_wb_kf.identify().unwrap();

  println!("Estimated value...");
  println!("Set Parameter:\t[jm, dm]={:?}", [jm, dm]);
  println!("ARX (BP):\t[jm, dm]={:?}", params_arx_lsm);
  println!("ARX (KF):\t[jm, dm]={:?}", params_arx_kf);
  println!("White box (BP):\t[jm, dm]={:?}", params_lsm_wb);
  println!("White box (KF):\t[jm, dm]={:?}", params_lsm_kf);

  data_storage.write_file().unwrap();
}


pub fn generator<T: num_traits::Float + std::ops::Add>(theta: &Vec<T>, x: T, x_noise: T, y_noise: T) -> T {
  let degree: usize = theta.len() - 1;
  let x_act = x + x_noise;
  let y: T = theta.iter().enumerate().fold(T::zero(), |a, (i, &b)| a + b * x_act.powi((degree - i) as i32));
  let y_sense: T = y + y_noise;
  y_sense
}

#[test]
fn test_polynomial_kalman() {
  const DATALEN: usize = 10000;

  let mut rng = rand::thread_rng();
  let x_sample: Normal<f64> = Normal::new(0.0, 0.5).unwrap();
  let noise_v: Normal<f64> = Normal::new(0.0, 0.01).unwrap();
  let noise_w: Normal<f64> = Normal::new(0.0, 0.02).unwrap();

  let mut theta: Vec<f64> = vec![0.2, 0.5];
  let degree: usize = theta.len() - 1;

  let mut s_kalman = kalman_filter::polynomial::KalmanFilter::<f64>::new(degree, 0.00001, 1.0, 10000.0);

  const ROW_SIZE: usize = 4;
  const DATA_SIZE: usize = 3 * DATALEN;
  const DATAILE_SEPARATOR: &str = ",";
  let output_filename: String = format!("data/out.csv");
  
  let mut data_storage = DataStorage::<f64, _, ROW_SIZE, DATA_SIZE>::new(output_filename, DATAILE_SEPARATOR);

  for i in 0..(3 * DATALEN) {
      
      if i < DATALEN { theta = vec![0.2, 0.5] }
      else if i < 2 * DATALEN { theta = vec![0.17, 0.5] }
      else if i < 3 * DATALEN { theta = vec![0.12, 0.5] };
      
      //theta[0] = 0.2 + 0.1 * (i  as f64 / (3.0 * DATALEN as f64));

      let x: f64 = 5.0 + x_sample.sample(&mut rng);
      let y: f64 = generator(&theta, x, noise_v.sample(&mut rng), noise_w.sample(&mut rng));
      s_kalman.update(x, y);

      let parameter: &Vec<f64> = s_kalman.parameter.data.as_vec();
      data_storage.add([x, y, parameter[0], parameter[1]]);
  }

  println!("{:?}", s_kalman.parameter);
  data_storage.write_file().unwrap();

}

#[test]
fn test_polynomial_batch() {
    let dx: f64 = 1e-3;

    let mut rng = rand::thread_rng();
    let noise_v: Normal<f64> = Normal::new(0.0, 0.01).unwrap();
    let noise_w: Normal<f64> = Normal::new(0.0, 0.02).unwrap();

    let theta: Vec<f64> = vec![0.2, 0.5, 1.2, -0.4, 0.3];
    let degree: usize = theta.len() - 1;

    let x: Vec<f64> = vec![0.0; 10000].iter().enumerate().map(|(i, _)| i as f64 * dx).collect();
    let y: Vec<f64> = x.iter().map(|&x| generator(&theta, x, noise_v.sample(&mut rng), noise_w.sample(&mut rng))).collect();

    let theta_hat: Vec<f64> = system_identification::lsm::polynomial::identify(&x, &y, degree).unwrap();
    
    println!("theta_origin:\t{theta:?}");
    println!("theta_est:\t{theta_hat:?}");
}