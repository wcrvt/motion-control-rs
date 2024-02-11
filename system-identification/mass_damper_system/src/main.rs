use rand_distr::{Distribution, Normal};

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
    let mut data_storage =
        DataStorage::<f64, _, ROW_SIZE, DATA_SIZE>::new(output_filename, DATAILE_SEPARATOR);

    for _ in 0..DATALEN {
        data_storage.add(sx_wb_kf.parameter.data);

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
        }

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
    println!(
        "White box (BP):\t[jm, dm]={:?}",
        sx_wb_lsm.identify().unwrap()
    );
    println!("White box (KF):\t[jm, dm]={:?}", sx_wb_kf.parameter.data);

    data_storage.write_file().unwrap();
}
