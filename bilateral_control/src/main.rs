use std::env;
use std::error::Error;

use digitalservo::plant::motor as plant;
use digitalservo::data_storage::DataStorage;
use digitalservo::mclib::disturbance_observer2 as dob;
use digitalservo::mclib::integrator;
use digitalservo::mclib::controller;

fn main() -> Result<(), Box<dyn Error>> {

    const MODE_LIM_U: usize = 3;
    let args: Vec<String> = env::args().collect();
    let mode: usize = if args.len() < 2 { 0 } else { if let Ok(i) = args[1].parse::<usize>() { std::cmp::min(i, MODE_LIM_U) } else { 0 } };
    let mode_str: &str = if mode == 0 { "4ch acceleration control based" }
    else if mode == 1 { "2ch admittance control based (position-based)" }
    else if mode == 2 { "2ch admittance control based (velocity-based)" }
    else { "2ch acceleration control based" };
    println!("Simulation mode: {}", mode_str);

    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 40000;
    const PLOOP_NUM: usize = 100;
    const TS: f64 = 50e-6;
    const TP: f64 = TS / PLOOP_NUM as f64;

    let kt: f64 = 1.20;
    let jm: f64 = 0.6;

    let g: f64 = 300.0;
    let mut dob = [dob::FirstOrder::new(TS, kt, jm, g); 2];
    let mut rfob = [dob::FirstOrder::new(TS, kt, jm, g); 2];

    //Logging
    const ROW_SIZE: usize = 5;
    const DATAILE_SEPARATOR: &str = ",";
    let output_filename: String = format!("data/out_mode_{}.csv", mode);
    let mut data_storage = DataStorage::<f64, _, ROW_SIZE, SLOOP_NUM>::new(output_filename, DATAILE_SEPARATOR);

    let env_k: f64 = 1000.0;
    let env_d: f64 = 20.0;
    let mut dis: [f64; 2] = [0.0; 2];

    let kp: f64 = 10000.0;
    let kd: f64 = 2.0 * kp.sqrt();
    let kf: f64 = 2.0 / jm * 2.0;

    let mut ddx_ref: [f64; 2] = [0.0; 2];
    let mut iq_ref: [f64; 2] = [0.0; 2];

    let mut integrator_first = integrator::FirstOrder::new(TS);
    let mut integrator_second = integrator::SecondOrder::new(TS);
    let mut pi_controller = [controller::PIController::new(kd, kp, TS); 2];

    let mut plant: [plant::Plant<f64>; 2] = [plant::Plant::new(TP, jm); 2];

    for _ in 0..SLOOP_NUM {

        let tau_dis: [f64; 2] = [dob[0].update(iq_ref[0], plant[0].d1x), dob[1].update(iq_ref[1], plant[1].d1x)];
        let tau_est: [f64; 2] = [rfob[0].update(iq_ref[0], plant[0].d1x), rfob[1].update(iq_ref[1], plant[1].d1x)];

        let i_cmp: [f64; 2] = [tau_dis[0] / kt, tau_dis[1] / kt];

        if mode == 0 {
            /* 4ch acceleration control based */
            let f_common: f64 = tau_est[0] + tau_est[1];
            let x_diff: f64 = plant[0].d0x - plant[1].d0x;
            let dx_diff: f64 = plant[0].d1x - plant[1].d1x;

            let ddx_common_ref: f64 = kf * (0.0 - f_common);
            let ddx_diff_ref: f64 = kp * (0.0 - x_diff) + kd * (0.0 - dx_diff);
            
            ddx_ref = [
                0.5 * (ddx_common_ref + ddx_diff_ref),
                0.5 * (ddx_common_ref - ddx_diff_ref)
            ];
        }
        else if mode == 1 {
            /* 2ch admittance control based (position-base)*/
            let f_common: f64 = tau_est[0] + tau_est[1];
            let ddx_common_ref: f64 = 0.5 * kf * (0.0 - f_common);
            let x_ref: f64 = integrator_second.update(ddx_common_ref);
            let dx_ref: f64 = integrator_first.update(ddx_common_ref);
            let ddx_ref_ff: f64 = ddx_common_ref;
    
            ddx_ref = [
                kp * (x_ref - plant[0].d0x) + kd * (dx_ref - plant[0].d1x) + ddx_ref_ff,
                kp * (x_ref - plant[1].d0x) + kd * (dx_ref - plant[1].d1x) + ddx_ref_ff,
            ];
        }
        else if mode == 2 {
            /* 2ch admittance control based (velocity-base)*/
            let f_common: f64 = tau_est[0] + tau_est[1];
            let ddx_common_ref: f64 = 0.5 * kf * (0.0 - f_common);
            let dx_ref: f64 = integrator_first.update(ddx_common_ref);
            let ddx_ref_ff: f64 = ddx_common_ref;
    
            ddx_ref = [
                pi_controller[0].calc(dx_ref, plant[0].d1x) + ddx_ref_ff,
                pi_controller[0].calc(dx_ref, plant[1].d1x) + ddx_ref_ff,
            ];
        }
        else if mode == 3 {
            /* 2ch acceleration control based */
            let f_common: f64 = tau_est[0] + tau_est[1];
            let ddx_common_ref: f64 = 0.5 * kf * (0.0 - f_common);
            ddx_ref = [
                ddx_common_ref - 10.0 * plant[0].d1x,
                ddx_common_ref - 10.0 * plant[1].d1x,
            ];
        }

        iq_ref[0] = (jm / kt) * ddx_ref[0] + i_cmp[0];
        iq_ref[1] = (jm / kt) * ddx_ref[1] + i_cmp[1];

        for _ in 0..PLOOP_NUM {
            dis[0] = if t < 0.2 { 0.0 } else if t < 0.6 { -1.0 } else if t < 1.0 { -2.0 } else if t < 1.4 { -1.0 } else { 0.0 };
            dis[1] = env_k * plant[1].d0x + env_d * plant[1].d1x;

            plant[0].update(kt * iq_ref[0] - dis[0]);
            plant[1].update(kt * iq_ref[1] - dis[1]);
   
            t += TP;
        }

        data_storage.add([t, plant[0].d0x, plant[1].d0x, dis[0], dis[1]]);
    }

    data_storage.write_file()?;

    Ok(())
}