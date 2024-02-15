use rand_distr::{Distribution, Normal};

use digitalservo::data_storage::DataStorage;
use digitalservo::system_identification::{kalman_filter, lsm};

fn main() {
    fn generator<T: num_traits::Float>(theta: &Vec<T>, x: T, x_noise: T, y_noise: T) -> T {
        let degree: usize = theta.len() - 1;
        let x_act = x + x_noise;
        let y: T = theta.iter().enumerate().fold(T::zero(), |a, (i, &b)| {
            a + b * x_act.powi((degree - i) as i32)
        });
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

    //Logging
    const DATA_SIZE: usize = 3 * DATALEN;
    const DATAILE_SEPARATOR: &str = ",";
    let output_filename: String = format!("data/out.csv");
    let mut data_storage = DataStorage::new(output_filename, DATAILE_SEPARATOR, DATA_SIZE);

    for i in 0..(3 * DATALEN) {
        /* fluctuating parameter */
        if i < DATALEN {
            theta = vec![0.2, 0.5]
        } else if i < 2 * DATALEN {
            theta = vec![0.17, 0.5]
        } else if i < 3 * DATALEN {
            theta = vec![0.12, 0.5]
        };

        /* fixed parameter */
        //theta = vec![0.17, 0.5];

        /* continuously increasing parameter */
        //theta[0] = 0.2 + 0.1 * (i  as f64 / (3.0 * DATALEN as f64));

        let x: f64 = 5.0 + x_sample.sample(&mut rng);
        let y: f64 = generator(
            &theta,
            x,
            noise_v.sample(&mut rng),
            noise_w.sample(&mut rng),
        );
        sx_kalman.update(x, y);
        sx_lsm.add(x, y);

        data_storage.add([x, y, sx_kalman.parameter[0], sx_kalman.parameter[1]]);
    }

    println!("{:?}", sx_kalman.parameter.data);
    println!("{:?}", sx_lsm.identify().unwrap());

    data_storage.write_file().unwrap();
}
