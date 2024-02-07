use std::error::Error;

use digitalservo::data_storage::DataStorage;
use digitalservo::plant::manipulator;
use digitalservo::observer::disturbance_observer as dob;

fn main()-> Result<(), Box<dyn Error>>  {

    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 20000;
    const PLOOP_NUM: usize = 100;
    const TS: f64 = 50e-6;
    const TP: f64 = TS / PLOOP_NUM as f64;

    let kt: [f64; manipulator::JOINTSPACE_DIM] = [1.20, 0.5, 0.5];
    let jm: [f64; manipulator::JOINTSPACE_DIM] = [2.5, 1.0, 0.5];
    let link: [f64; manipulator::JOINTSPACE_DIM] = [0.5, 0.4, 0.4];

    let plant_init_theta: [f64; manipulator::JOINTSPACE_DIM] = [-0.2, 1.2, 1.0];
    let mut plant = manipulator::SeriesLinkManipulator::new(kt, jm, link, TP)
        .set_init_theta(plant_init_theta);
    let plant_init_x: [f64; manipulator::WORKSPACE_DIM] = plant.d0x.data;

    let g: f64 = 500.0;
    let mut dob = [
        dob::VelocityBased::<_, 0>::new(TS, kt[0], jm[0], g),
        dob::VelocityBased::<_, 0>::new(TS, kt[1], jm[1], g),
        dob::VelocityBased::<_, 0>::new(TS, kt[2], jm[2], g),
    ];

    //Logging
    const ROW_SIZE: usize = 8;
    const DATAILE_SEPARATOR: &str = ",";
    let output_filename: String = format!("data/out.csv");
    let mut data_storage = DataStorage::<f64, _, ROW_SIZE, SLOOP_NUM>::new(output_filename, DATAILE_SEPARATOR);

    //Controller
    let kp: f64 = 1000.0;
    let kd: f64 = 2.0 * kp.sqrt();

    let mut iq_ref: [f64; manipulator::JOINTSPACE_DIM] = [0.0; manipulator::JOINTSPACE_DIM];
    let mut i_cmp: [f64; manipulator::JOINTSPACE_DIM] = [0.0; manipulator::JOINTSPACE_DIM];
    let mut tau_dis: [f64; manipulator::JOINTSPACE_DIM] = [0.0; manipulator::JOINTSPACE_DIM];

    let xcmd_amp: [f64; manipulator::WORKSPACE_DIM] = [0.1, 0.1];
    let xcmd_freq: f64 = 3.0 * (2.0 * std::f64::consts::PI);
    let xcmd_st: [f64; manipulator::WORKSPACE_DIM] = [0.0, 1.0 / xcmd_freq / 2.0 * std::f64::consts::PI];

    for _ in 0..SLOOP_NUM {

        /* disturbance observer */
        for i in 0..manipulator::JOINTSPACE_DIM {
            tau_dis[i] = dob[i].update(iq_ref[i], plant.d1theta[i]);
            i_cmp[i] = tau_dis[i] / kt[i];            
        }

        /* work-space trajectory */
        let xcmd: [f64; manipulator::WORKSPACE_DIM] = [
            if t > xcmd_st[0] {plant_init_x[0] + xcmd_amp[0] * 0.5 * (1.0 - (xcmd_freq * (t - xcmd_st[0])).cos()) } else { plant_init_x[0] },
            if t > xcmd_st[1] {plant_init_x[1] + xcmd_amp[1] * 0.5 * (1.0 - (xcmd_freq * (t - xcmd_st[1])).cos()) } else { plant_init_x[1] },
        ];
        let dx_cmd: [f64; manipulator::WORKSPACE_DIM] = [
            if t > xcmd_st[0] {xcmd_amp[0] * 0.5 * xcmd_freq.powi(1) * (xcmd_freq * (t - xcmd_st[0])).sin()} else {0.0},
            if t > xcmd_st[1] {xcmd_amp[0] * 0.5 * xcmd_freq.powi(1) * (xcmd_freq * (t - xcmd_st[1])).sin()} else {0.0},
        ];
        let ddx_cmd: [f64; manipulator::WORKSPACE_DIM] = [
            if t > xcmd_st[0] {xcmd_amp[0] * 0.5 * xcmd_freq.powi(2) * (xcmd_freq * (t - xcmd_st[0])).cos()} else {0.0},
            if t > xcmd_st[1] {xcmd_amp[1] * 0.5 * xcmd_freq.powi(2) * (xcmd_freq * (t - xcmd_st[1])).cos()} else {0.0},
        ];

        /* work-space acceleration reference */
        let ddx_ref: [f64; manipulator::WORKSPACE_DIM] = [
            kp * (xcmd[0] - plant.d0x[0]) + kd * (dx_cmd[0] - plant.d1x[0]) + ddx_cmd[0],
            kp * (xcmd[1] - plant.d0x[1]) + kd * (dx_cmd[1] - plant.d1x[1]) + ddx_cmd[1],
        ];

        let ddx_null: [f64; manipulator::JOINTSPACE_DIM] = [
            0.0 * (plant.d0theta[1] - plant.d0theta[0]) - 200.0 * plant.d1theta[0],
            0.0 * (plant.d0theta[2] - plant.d0theta[1]) - 200.0 * plant.d1theta[1],
            10000.0 * (0.0 - plant.d0theta[2]) - 200.0 * plant.d1theta[2],
        ];

        /* joint-space acceleration reference */
        let ddtheta_ref: [f64; manipulator::JOINTSPACE_DIM] = plant.computed_torque_method(&ddx_ref, &ddx_null);

        for i in 0..manipulator::JOINTSPACE_DIM {
            iq_ref[i] = (jm[i] / kt[i]) * ddtheta_ref[i] + i_cmp[i];
        }

        for _ in 0..PLOOP_NUM {
            let dis = [0.0, 0.0, 0.0];
            plant.update(iq_ref, dis);
            t += TP;
        }

        data_storage.add([t, xcmd[0], xcmd[1], plant.d0x[0], plant.d0x[1], plant.d0theta[0], plant.d0theta[1], plant.d0theta[2]]);
    }

    data_storage.write_file()?;

    Ok(())

}
