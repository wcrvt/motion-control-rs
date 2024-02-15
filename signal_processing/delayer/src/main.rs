use digitalservo::data_storage::DataStorage;
use digitalservo::signal::delayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 20000;
    const TS: f64 = 100e-6;

    //Logging
    const DATAFILE_SEPARATOR: &str = ",";
    const DATAFILE_PATH: &str = "data/out.csv";
    let mut data_storage = DataStorage::new(DATAFILE_PATH, DATAFILE_SEPARATOR, SLOOP_NUM);

    const DELAY_TIME: f64 = 0.1;
    const DELAYER_BUFSIZE: usize = (DELAY_TIME / TS) as usize;
    let mut delayer = delayer::Delayer::<f64, DELAYER_BUFSIZE>::new();

    for _ in 0..SLOOP_NUM {
        //let u: f64 = if t < 0.5 { 0.0 } else { 1.0 };
        let u: f64 = (2.0 * std::f64::consts::PI * t).sin();
        let y: f64 = delayer.output(u);

        //Logging
        data_storage.add([t, u, y]);

        t += TS;
    }

    data_storage.write_file()?;

    Ok(())
}
