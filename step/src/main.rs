use digitalservo::signal::freefilter;
use digitalservo::data_storage::DataStorage;

fn main() {

    let dlen: usize = 50000;

    let ts: f64 = 1e-6;
    let omega: f64 = 200.0;
    let mut filter1 = freefilter::FreeFilter::new(&[2.0 * omega], &[1.0, 2.0 * omega], ts);
    let mut filter2 = freefilter::FreeFilter::new(&[2.0 * omega, omega * omega], &[1.0, 2.0 * omega, omega * omega], ts);
    let mut filter3 = freefilter::FreeFilter::new(&[0.0, omega * omega], &[1.0, 2.0 * omega, omega * omega], ts);
    let mut data_storage = DataStorage::new("./out.csv", ",", dlen);

    let mut t: f64 = 0.0;
    for _ in 0..dlen {
        let x: f64 = 1.0;
        let y1: f64 = filter1.update(x);
        let y2: f64 = filter2.update(x);
        let y3: f64 = filter3.update(x);
        t += ts;
        data_storage.add([t, x, y1, y2, y3]);
    }

    data_storage.write_file().unwrap();

}
