use digitalservo::data_storage::DataStorage;
use digitalservo::signal::freefilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 50000;
    const TS: f64 = 1e-6;

    let omega: f64 = 200.0;
    let mut filter1 = freefilter::FreeFilter::new(&[2.0 * omega], &[1.0, 2.0 * omega], TS);
    let mut filter2 = freefilter::FreeFilter::new(&[2.0 * omega, omega * omega], &[1.0, 2.0 * omega, omega * omega], TS);
    let mut filter3 = freefilter::FreeFilter::new(&[0.0, omega * omega], &[1.0, 2.0 * omega, omega * omega], TS);

    //Logging
    const DATAFILE_SEPARATOR: &str = ",";
    const DATAFILE_PATH: &str = "data/out.csv";
    let mut data_storage = DataStorage::new(DATAFILE_PATH, DATAFILE_SEPARATOR, SLOOP_NUM);

    for _ in 0..SLOOP_NUM {
        let x: f64 = 1.0;
        let y1: f64 = filter1.update(x);
        let y2: f64 = filter2.update(x);
        let y3: f64 = filter3.update(x);
        t += TS;
        data_storage.add([t, x, y1, y2, y3]);
    }

    data_storage.write_file()?;

    Ok(())
}
