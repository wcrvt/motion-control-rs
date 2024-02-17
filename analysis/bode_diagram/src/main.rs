use digitalservo::*;
use digitalservo::analysis::freqeucy_response::*;

fn main() {
    const DATALEN: usize = 10000;
    let mut data_storage = DataStorage::new("data/bode.csv", ",", DATALEN);

    // let freq: f64 = 10.0;
    // let omega = 2.0 * std::f64::consts::PI * freq;
    // let numer = [1.0, omega];
    // let denom = [omega];
    // let b = get_frequency_characteristics_from_s(numer, denom, 0.0, 500.0, DATALEN);
    
    // for res in b {
    //     data_storage.add([res.freq, 20.0 * res.gain.log10(), res.phase * 180.0 / std::f64::consts::PI]);
    // }

    let numer = [0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];
    let denom: [f64; 1] = [1.0];
    let b = get_frequency_characteristics_from_z(numer, denom, 1e-1, 0.0, 100.0, DATALEN);
    
    for res in b {
        data_storage.add([res.freq, 20.0 * res.gain.log10(), res.phase * 180.0 / std::f64::consts::PI]);
    }

    data_storage.write_file().unwrap();
}
