pub mod serial;

use serial::canadapter::CANAdapter;
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canadapter = CANAdapter::open()?;

    let channel: u8 = 2;

    canadapter.drive_enable(channel)?;
    thread::sleep(time::Duration::from_millis(200));

    const TS_MS: u64 = 10;
    const V_AMP: f64 = 800.0;
    const V_FREQ: f64 = 2.0;

    const T_PERIOD_STEADY: f64 = 2.0;
    const T_PERIOD_ACC: f64 = 1.0 / (2.0 * V_FREQ);
    
    const T_ACC_START: f64 = 0.0;
    const T_STEADY_START: f64 = T_ACC_START + T_PERIOD_ACC;
    const T_DEACC_START: f64 = T_STEADY_START + T_PERIOD_STEADY;
    const T_END: f64 = T_DEACC_START + T_PERIOD_ACC;

    const PI2: f64 = 2.0 * std::f64::consts::PI;

    const ITERLOOP: usize = (T_END * 1000.0 / (TS_MS as f64)) as usize + 10;

    for i in 0..ITERLOOP {
        let t: f64 = (TS_MS as f64) * 1e-3 * (i as f64);

        let value: f64 = if t < T_STEADY_START {
            0.5 * V_AMP * (1.0 - (PI2 * V_FREQ * t).cos())
        } else if t < T_DEACC_START {
            V_AMP
        } else if t < T_END {
            0.5 * V_AMP * (1.0 + (PI2 * V_FREQ * (t - T_DEACC_START)).cos())
        } else {
            0.0
        };

        canadapter.send_reference(channel, value)?;
        thread::sleep(time::Duration::from_millis(TS_MS));
    }

    canadapter.drive_disable(channel)?;
    thread::sleep(time::Duration::from_millis(200));

    canadapter.close()?;

    Ok(())
}
