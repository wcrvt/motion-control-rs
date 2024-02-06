use digitalservo::mclib::disturbance_observer;
use digitalservo::plant::motor;
use digitalservo::data_storage::DataStorage;

const TEST_NUM: usize = 4;

fn main()-> Result<(), Box<dyn std::error::Error>> {

    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 500;
    const PLOOP_NUM: usize = 100;
    const TS: f64 = 500e-6;
    const TP: f64 = TS / PLOOP_NUM as f64;

    //Logging
    const ROW_SIZE: usize = TEST_NUM + 2;
    const DATAFILE_SEPARATOR: &str = ",";
    const DATAFILE_PATH: &str = "data/estimated.csv";
    let mut data_storage = DataStorage::<f64, _, ROW_SIZE, SLOOP_NUM>::new(DATAFILE_PATH, DATAFILE_SEPARATOR);

    //Plant
    let kt: f64 = 1.2;
    let jm: f64 = 0.4;
    let mut plant = motor::Plant::new(TP, jm);

    //Disturbance observer
    let g: f64 = 100.0;
    let mut dob0 = disturbance_observer::VelocityBased::<_, 0>::new(TS, kt, jm, g);
    let mut dob1 = disturbance_observer::VelocityBased::<_, 1>::new(TS, kt, jm, g);
    let mut dob2 = disturbance_observer::VelocityBased::<_, 2>::new(TS, kt, jm, g);
    let mut dob3 = disturbance_observer::VelocityBased::<_, 3>::new(TS, kt, jm, g);

    //Control signal
    let mut iq_ref: f64 = 0.0;
    let mut tau: f64;
    let mut tau_dis: f64;
    let mut tau_dis_est: [f64; TEST_NUM] = [0.0; TEST_NUM];

    for _ in 0..SLOOP_NUM {

        /* disturbance observer */
        tau_dis_est[0] = dob0.update(iq_ref, plant.d1x);
        tau_dis_est[1] = dob1.update(iq_ref, plant.d1x);
        tau_dis_est[2] = dob2.update(iq_ref, plant.d1x);
        tau_dis_est[3] = dob3.update(iq_ref, plant.d1x);

        iq_ref = -1.0 + 0.5 * (2.0 * std::f64::consts::PI * 3.0 * t);
        if t > 0.15 { iq_ref = - 1.0; }

        tau = kt * iq_ref;

        tau_dis = 1.0;

        for _ in 0..PLOOP_NUM {
            plant.update(tau - tau_dis);
            t += TP;
        }

        data_storage.add([t, tau_dis, tau_dis_est[0], tau_dis_est[1], tau_dis_est[2], tau_dis_est[3]]);
    }

    data_storage.write_file()?;

    Ok(())

}
