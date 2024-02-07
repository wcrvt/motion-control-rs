use digitalservo::signal::freefilter;
use digitalservo::data_storage::DataStorage;


fn main()-> Result<(), Box<dyn std::error::Error>> {

    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 20000;
    const TS: f64 = 100e-6;

    //Logging
    const ROW_SIZE: usize = 3;
    const DATAFILE_SEPARATOR: &str = ",";
    const DATAFILE_PATH: &str = "data/out.csv";
    let mut data_storage = DataStorage::<f64, _, ROW_SIZE, SLOOP_NUM>::new(DATAFILE_PATH, DATAFILE_SEPARATOR);

    let omega: f64 = 10.0;
    let numer: [f64; 2] = [2.0 * omega, omega.powi(2)];
    let denom: [f64; 3] = [1.0, 2.0 * omega, omega.powi(2)];
    let mut filter = freefilter::FreeFilter::new(&numer, &denom, TS);

    for _ in 0..SLOOP_NUM {

        let u: f64 = 1.0;// + (omega * t).sin() + (0.2 * omega * t).sin() ;
        let y: f64 = filter.update(u);

        //Logging
        data_storage.add([t, u, y]);

        t += TS;
    }

    data_storage.write_file()?;

    Ok(())

}
