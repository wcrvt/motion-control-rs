use std::error::Error;

use digitalservo::data_storage::DataStorage;
use digitalservo::plant::pendulum;
use digitalservo::observer::disturbance_observer as dob;

fn main() -> Result<(), Box<dyn Error>> {

    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 10000;
    const PLOOP_NUM: usize = 100;
    const TS: f64 = 500e-6;
    const TP: f64 = TS / PLOOP_NUM as f64;

    //Motor
    let kt:f64 = 1.2;
    let mm: f64 = 0.5;
    
    //Pendulum
    let lp: f64 = 2.0;
    let mp: f64 = 0.10;
    let jp: f64 = mp * lp.powi(2) / 12.0;
    let k_acc: f64 = (jp + mp * lp.powi(2)) / (mp * lp);

    let mut plant = pendulum::Pendulum::new(TP)
        .set_motor_param(kt, mm)
        .set_pendulum_param(lp, mp)
        .set_init_theta(0.2);

    let g: f64 = 500.0;
    let mut dob = dob::VelocityBased::<_, 0>::new(TS, kt, mm + mp, g);


    //Logging
    const ROW_SIZE: usize = 5;
    const DATAILE_SEPARATOR: &str = ",";
    let output_filename: String = format!("data/out.csv");
    let mut data_storage = DataStorage::<f64, _, ROW_SIZE, SLOOP_NUM>::new(output_filename, DATAILE_SEPARATOR);

    //Controller
    let kp: f64 = 1000.0;
    let _kd: f64 = 2.0 * kp.sqrt();

    let mut iq_ref: f64 = 0.0;
    let mut i_cmp: f64;
    let mut tau_dis: f64;

    let mut ddxm_ref: f64;

    let mut err: f64;
    let mut ierr: f64 = 0.0;

    let omega_c: f64 = 100.0;
    let ki: f64 = omega_c.powi(3);
    let kp: f64 = 3.0 * omega_c.powi(2);
    let kd: f64 = 3.0 * omega_c.powi(1);

    let mut theta_cmd: f64;

    for _ in 0..SLOOP_NUM {

        /* disturbance observer */
        tau_dis = dob.update(iq_ref, plant.d1xm);
        i_cmp = tau_dis / kt;

        /* angle controller */
        theta_cmd = if t < 1.0 { 0.2 } else if t < 4.0 { - 0.2 } else { 0.0 };
        err = theta_cmd - plant.d0theta;
        ierr += err * TS;

        ddxm_ref = - kp * plant.d0theta - kd * plant.d1theta + ki * ierr;     

        iq_ref = (k_acc * (mm + mp) / kt) * ddxm_ref + i_cmp;

        for _ in 0..PLOOP_NUM {
            plant.update(iq_ref, 0.0);
            t += TP;
        }

        data_storage.add([t, plant.d0xm, plant.d0xp[0], plant.d0xp[1], plant.d0theta]);
    }

    data_storage.write_file()?;

    Ok(())

}
