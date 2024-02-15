use rand::Rng;
use digitalservo::data_storage::DataStorage;
use digitalservo::system_identification::gpr;

fn f(x: f64) -> f64 {
    1.0 * (7.0 * x).sin()
    + 2.0 * (6.0 * x).sin().cos()
    + 0.5 * (-0.2 * x).exp() * (5.0 * x).sin() * (2.8 * x).cos()
}

fn kernel(x1: f64, x2: f64) -> f64 {
    let theta1: f64 = 20.0;
    let theta2: f64 = 0.1;
    let theta3: f64 = 0.1;
    let theta4: f64 = 0.01;
    let delta: f64 = if x1 == x2 { 1.0 } else { 0.0 };
    theta1 * (- (x1 - x2).powi(2) / theta2).exp() + (delta * theta3) + theta4
}

fn main() {

    const SAMPLES: usize = 50;
    const RANGE_X: f64 = 5.0;
    const DATA_MAPPING: usize = 500;

    //Logging
    const DATAFILE_SEPARATOR: &str = ",";

    const ROW_SIZE_SAMPLE: usize = 2;
    const DATAFILE_PATH_SAMPLE: &str = "data/sample.csv";
    let mut data_storage_sample = DataStorage::<f64, _, ROW_SIZE_SAMPLE, SAMPLES>::new(DATAFILE_PATH_SAMPLE, DATAFILE_SEPARATOR);
    
    const ROW_SIZE_MAP: usize = 4;
    const DATAFILE_PATH_MAP: &str = "data/map.csv";
    let mut data_storage_map = DataStorage::<f64, _, ROW_SIZE_MAP, DATA_MAPPING>::new(DATAFILE_PATH_MAP, DATAFILE_SEPARATOR);

    let mut gpr = gpr::GaussianProcessRegression::new(kernel, 0.5);

    //Samples
    let mut sample_x: [f64; SAMPLES] = [0.0; SAMPLES];
    let mut sample_y: [f64; SAMPLES] = [0.0; SAMPLES];

    /* Random sample */
    let mut rng = rand::thread_rng();
    for i in 0..SAMPLES {
        sample_x[i] = rng.gen::<f64>() * RANGE_X;
        sample_y[i] = f(sample_x[i]) + (rng.gen::<f64>() - 0.5) * 0.5;

        gpr.add(sample_x[i], sample_y[i]);
        data_storage_sample.add([sample_x[i], sample_y[i]]);
    }

    //Mapping (prediction)
    let dx: f64 = (gpr.x_max - gpr.x_min) / DATA_MAPPING as f64;
    for i in 0..DATA_MAPPING {
        let x: f64 = gpr.x_min + dx * i as f64;
        let y: f64 = f(x);
        let predict = gpr.predict(x);
        data_storage_map.add([x, y, predict.mean, predict.stdev]);
    }

    //Write file
    data_storage_sample.write_file().unwrap();
    data_storage_map.write_file().unwrap();

}
