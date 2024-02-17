use digitalservo::*;

fn main() {
    const DATALEN: usize = 10000;
    let mut data_storage = DataStorage::new("data/nyquist.csv", ",", DATALEN);

    use digitalservo::analysis::freqeucy_response::*;
    let freq: f64 = 10.0;
    let omega = 2.0 * std::f64::consts::PI * freq;
    let numer = [omega.powi(2)];
    let denom = [1.0, 2.0 * omega, omega.powi(2)];
    let b = get_frequency_characteristics_from_s(numer, denom, 0.0, 500.0, DATALEN);
    
    for res in b {
        data_storage.add([res.freq, res.re, res.im]);
    }

    data_storage.write_file().unwrap();
}
