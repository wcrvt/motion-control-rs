use digitalservo::analysis::freqeucy_response::*;

fn main() {
    const DATALEN: usize = 10000;

    let freq: f64 = 10.0;
    let omega: f64 = 2.0 * std::f64::consts::PI * freq;
    let zeta: f64 = 5.0f64.sqrt();

    //closed-loop
    // let numer = [2.0 * zeta * omega, omega.powi(2)];
    // let denom = [1.0, 2.0 * zeta * omega, omega.powi(2)];

    //open-loop
    let numer = [2.0 * zeta * omega, omega.powi(2)];
    let denom = [1.0, 0.0, 0.0];

    let mut storage = FrequencyAnalyzer::<f64, _>::new("data/frequency_response_s.csv", ",", DATALEN);
    storage.frequency_response_s(numer, denom);

    let numer = [0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];
    let denom: [f64; 1] = [1.0];
    let mut storage = FrequencyAnalyzer::<f64, _>::new("data/frequency_response_z.csv", ",", DATALEN);
    storage.frequency_response_z(numer, denom, 1e-2);
}
