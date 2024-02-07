pub mod serial;

use std::{thread, time};
use serial::canadapter::CANAdapter;

const TS_MS: u64 = 5;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut canadapter = CANAdapter::open()?;

    canadapter.drive_enable(1)?;
    thread::sleep(time::Duration::from_millis(200));

    for i in 0..501 {
        let freq = 1.0;
        let value: f64 = 500.0 * (((TS_MS as f64) * 1e-3) * (i as f64) * freq * (2.0 * std::f64::consts::PI)).sin();
        canadapter.send_reference(1, value)?;
        thread::sleep(time::Duration::from_millis(TS_MS));
    }

    canadapter.drive_disable(1)?;
    thread::sleep(time::Duration::from_millis(200));

    canadapter.close()?;

    Ok(())
}
