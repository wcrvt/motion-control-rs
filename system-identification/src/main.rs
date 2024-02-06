use rand_distr::{Normal, Distribution};

use digitalservo::data_storage::DataStorage;
use digitalservo::system_identification::{kalman_filter, lsm};

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

  let mut sx_arx_kf = kalman_filter::arx::KalmanFilter::<_, 1, 0>::new(0.1, 0.1, 10000.0);
  let mut sx_wb_kf = kalman_filter::whitebox::KalmanFilter::<_, 2>::new(0.1, 1.0, 10000.0);

  let mut sx_arx_lsm = lsm::arx::DataBuffer::<f64, 1, 0>::new();
  let mut sx_wb_lsm = lsm::whitebox::DataBuffer::<f64, 2>::new();

  //Data storage
  const ROW_SIZE: usize = 2;
  const DATA_SIZE: usize = DATALEN;
  const DATAILE_SEPARATOR: &str = ",";
  let output_filename: String = format!("data/out.csv");
  let mut data_storage = DataStorage::<f64, _, ROW_SIZE, DATA_SIZE>::new(output_filename, DATAILE_SEPARATOR);

  for _ in 0..DATALEN {

    data_storage.add([0.0, 0.0]);

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
    sx_arx_lsm.add(uref_sampled, omega_sampled, omega_sensed);
    sx_wb_lsm.add(&[acc_sensed, omega_sensed], uref_sampled);

    sx_arx_kf.update(uref_sampled, omega_sampled, omega_sensed);
    sx_wb_kf.update(&[acc_sensed, omega_sensed], uref_sampled);

    t += TS;
  }
  
  println!("\nSampling time: {TS:?}\n");
  println!("Estimated value...");
  println!("Set Parameter:\t[jm, dm]={:?}", [jm, dm]);
  println!("ARX (BP):\t[jm, dm]={:?}", sx_arx_lsm.identify().unwrap());
  println!("ARX (KF):\t[jm, dm]={:?}", sx_arx_kf.parameter.data);
  println!("White box (BP):\t[jm, dm]={:?}", sx_wb_lsm.identify().unwrap());
  println!("White box (KF):\t[jm, dm]={:?}", sx_wb_kf.parameter.data);

  data_storage.write_file().unwrap();
}



#[test]
fn test_polynomial_identification() {

  fn generator<T: num_traits::Float>(theta: &Vec<T>, x: T, x_noise: T, y_noise: T) -> T {
    let degree: usize = theta.len() - 1;
    let x_act = x + x_noise;
    let y: T = theta.iter().enumerate().fold(T::zero(), |a, (i, &b)| a + b * x_act.powi((degree - i) as i32));
    let y_sense: T = y + y_noise;
    y_sense
  }
  

  const DATALEN: usize = 10000;

  let mut rng = rand::thread_rng();
  let x_sample: Normal<f64> = Normal::new(0.0, 0.5).unwrap();
  let noise_v: Normal<f64> = Normal::new(0.0, 0.01).unwrap();
  let noise_w: Normal<f64> = Normal::new(0.0, 0.02).unwrap();

  let mut theta: Vec<f64> = vec![0.2, 0.5];
  const DEGREE: usize = 1;

  let mut sx_kalman = kalman_filter::polynomial::KalmanFilter::<_, DEGREE>::new(0.00001, 1.0, 10000.0);
  let mut sx_lsm = lsm::polynomial::DataBuffer::<f64, DEGREE>::new();

  const ROW_SIZE: usize = 4;
  const DATA_SIZE: usize = 3 * DATALEN;
  const DATAILE_SEPARATOR: &str = ",";
  let output_filename: String = format!("data/out.csv");
  
  let mut data_storage = DataStorage::<f64, _, ROW_SIZE, DATA_SIZE>::new(output_filename, DATAILE_SEPARATOR);

  for i in 0..(3 * DATALEN) {
    /* fluctuating parameter */
    if i < DATALEN { theta = vec![0.2, 0.5] }
    else if i < 2 * DATALEN { theta = vec![0.17, 0.5] }
    else if i < 3 * DATALEN { theta = vec![0.12, 0.5] };

    /* fixed parameter */
    //theta = vec![0.17, 0.5];
    
    /* continuously increasing parameter */
    //theta[0] = 0.2 + 0.1 * (i  as f64 / (3.0 * DATALEN as f64));

    let x: f64 = 5.0 + x_sample.sample(&mut rng);
    let y: f64 = generator(&theta, x, noise_v.sample(&mut rng), noise_w.sample(&mut rng));
    sx_kalman.update(x, y);
    sx_lsm.add(x, y);

    data_storage.add([x, y, sx_kalman.parameter[0], sx_kalman.parameter[1]]);
  }

  println!("{:?}", sx_kalman.parameter.data);
  println!("{:?}", sx_lsm.identify().unwrap());
  
  data_storage.write_file().unwrap();

}