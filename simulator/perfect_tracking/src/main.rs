use std::error::Error;

use digitalservo::state_space::{continuous, discrete, blocked};
use digitalservo::signal::stable_inversion::StableInverter;
use digitalservo::data_storage::DataStorage;

const PLANT_ORDER: usize = 2;

fn main() -> Result<(), Box<dyn Error>>{

    const SAMPLES: usize = 2000;
    const SLOOP_NUM: usize = SAMPLES * PLANT_ORDER;
    const PLOOP_NUM: usize = 10;
    const TS: f64 = 1e-4;
    const TM: f64 = TS * PLANT_ORDER as f64;
    const TP: f64 = TS / PLOOP_NUM as f64;

    let mut t: f64 = 0.0;

    //Logging
    const ROW_SIZE: usize = 3;
    const DATAILE_SEPARATOR: &str = ",";
    let output_filename: String = format!("data/out.csv");
    let mut data_storage = DataStorage::<f64, _, ROW_SIZE, SLOOP_NUM>::new(output_filename, DATAILE_SEPARATOR);

    let a_c: [[f64; 2]; 2] = [[0.0, 1.0], [-1000.0, -20.0]];
    let b_c: [f64; 2] = [0.0, 1.0];
    let c_c: [f64; 2] = [1.0, -1.0 / 50.0];
    let c_ssr = continuous::SSR::new(&a_c, &b_c, &c_c);
    let d_ssr = discrete::SSR::from_continuous_ssr(&c_ssr, TS);
    let mut b_ssr = blocked::BlockedSSR::from_discrete_ssr(&d_ssr);

    //Reference
    pub fn reference(t: f64) -> [f64; 2] {
        let t0: f64 = 0.3;
        let freq: f64 = 20.0;
        let omega: f64 = 2.0 * std::f64::consts::PI * freq;
        let te: f64 = t0 + 1.0 / freq;

        let x: f64 = if t < t0 || t > te { 0.0 } else { 0.5 * (1.0 - (omega * (t - t0)).cos()) };
        let v: f64 = if t < t0 || t >te { 0.0 } else { 0.5 * omega * (omega * (t - t0)).sin() };
        [x, v]
    }

    const T_MAX: f64 = 0.4;
    fn f_unstable(t: f64) -> f64 { 50.0 * (- 50.0 * t).exp()}
    let mut stable_inverter = StableInverter::new(reference, None, Some(f_unstable), T_MAX);

    //Calcularate feedforward signal
    let mut ref_z1: [f64; 2];
    let mut ref_z0: [f64; 2] = [0.0; 2];
    let mut u_ff: [f64; SLOOP_NUM] = [0.0; SLOOP_NUM];
    for i in 0..SAMPLES {
        ref_z1 = stable_inverter.output(t + TM);
        let u_ff_block: [f64; PLANT_ORDER] = b_ssr.calculate_input(&ref_z0, &ref_z1);
        for j in 0..PLANT_ORDER {
            u_ff[PLANT_ORDER * i + j] = u_ff_block[j];
        }
        ref_z0 = ref_z1;
        t += TM;
    }

    //Simulator
    t = 0.0;
    let mut c_plant = continuous::Plant::new(&c_ssr, TP);
    for i in 0..SLOOP_NUM {
        for _ in 0..PLOOP_NUM {
            c_plant.update(u_ff[i]);
        }
        t += TS;
        
        data_storage.add([t, reference(t)[0], c_plant.y]);
    } 

    data_storage.write_file()?;

    Ok(())
}
