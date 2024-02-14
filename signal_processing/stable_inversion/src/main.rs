use digitalservo::data_storage::DataStorage;
use digitalservo::signal::stable_inversion::StableInverter;

const N: usize = 3;

fn main() {

    const SLOOP_NUM: usize = 2000;
    const TS: f64 = 1e-4;
    let mut t: f64 = 0.0;

    //Logging
    const ROW_SIZE: usize = 4;
    const DATAILE_SEPARATOR: &str = ",";
    let output_filename: String = format!("data/out.csv");
    let mut data_storage = DataStorage::<f64, _, ROW_SIZE, SLOOP_NUM>::new(output_filename, DATAILE_SEPARATOR);

    //reference
    fn reference(t: f64) -> [f64; N] {
        let t0: f64 = 0.05;
        let freq: f64 = 20.0;
        let omega: f64 = 2.0 * std::f64::consts::PI * freq;
        let te: f64 = t0 + 1.0 / freq;

        let x: f64 = if t < t0 || t > te { 0.0 } else { 0.5 * (1.0 - (omega * (t - t0)).cos()) };
        let v: f64 = if t < t0 || t > te { 0.0 } else { 0.5 * omega * (omega * (t - t0)).sin() };
        let a: f64 = if t < t0 || t > te { 0.0 } else { 0.5 * omega * omega * (omega * (t - t0)).cos() };
        [x, v, a]
    }
    
    const T_MAX: f64 = 0.4;
    const OMEGA: f64 = 40.0;
    let zeros: [f64; N] = [1.0, 0.0, -1.0 / (OMEGA * OMEGA)];
    fn f_stable(t: f64) -> f64 { (-OMEGA * t).exp() / 2.0 * OMEGA }    
    fn f_unstable(t: f64) -> f64 { (-OMEGA * t).exp() / 2.0 * OMEGA }

    let mut stable_inverter = StableInverter::new(reference, Some(f_stable), Some(f_unstable), T_MAX);
    
    for _ in 0..SLOOP_NUM {
        let z: [f64; N] = reference(t);
        let x: [f64; N] = stable_inverter.output(t);

        let mut y: f64 = 0.0;
        for i in 0..N {
            y += zeros[i] * x[i];
        }

        data_storage.add([t, z[0], x[0], y]);

        t += TS;
    }
    
    data_storage.write_file().unwrap();
}
