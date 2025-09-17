#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use digitalservo::data_storage::DataStorage;
use digitalservo::signal::differentiator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Time step configuration
    let mut t: f64 = 0.0;
    const SLOOP_NUM: usize = 20000;
    const TS: f64 = 100e-6;

    //Logging
    const DATAFILE_SEPARATOR: &str = ",";
    const DATAFILE_PATH: &str = "data/out.csv";
    let mut data_storage = DataStorage::new(DATAFILE_PATH, DATAFILE_SEPARATOR, SLOOP_NUM);

    let g: f64 = 100.0;
    let mut differentiator = differentiator::Differentiator::<_, 3, 0>::new(TS, g);

    for _ in 0..SLOOP_NUM {
        let u: f64 = t.powi(3);
        let y: f64 = differentiator.update(u);

        //Logging
        data_storage.add([t, u, y]);

        t += TS;
    }

    data_storage.write_file()?;

    Ok(())
}
