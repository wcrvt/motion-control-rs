//https://users.rust-lang.org/t/how-to-save-a-vector-of-float-64/93728/4

use std::io::{BufWriter, Write};
use std::{error::Error, fs::File, path::Path};

pub struct DataStorage<T, P, const ROWSIZE: usize, const DATALEN: usize> {
    data: [[T; ROWSIZE]; DATALEN],
    path: P,
    separator: String,
    cnt: usize,
}

impl<T, P, const ROWSIZE: usize, const DATALEN: usize> DataStorage<T, P, ROWSIZE, DATALEN>
where
    T: std::fmt::Display
        + std::fmt::Debug
        + num_traits::Num
        + std::clone::Clone
        + std::marker::Copy,
    P: AsRef<Path>,
{
    pub fn new(path: P, separator: &str) -> Self {
        Self {
            data: [[T::zero(); ROWSIZE]; DATALEN],
            path,
            separator: separator.into(),
            cnt: 0,
        }
    }

    pub fn add(&mut self, data: [T; ROWSIZE]) {
        if self.cnt < DATALEN {
            self.data[self.cnt] = data;
            self.cnt += 1;
        }
    }

    pub fn write_file(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(&self.path.as_ref())?;
        let mut writer = BufWriter::new(file);

        for x in &self.data {
            let data_row = x
                .iter()
                .map(|x| format!("{:.06?}", x))
                .reduce(|x, y| x + &self.separator + &y)
                .unwrap();
            write!(writer, "{data_row}\n")?;
            writer.flush()?;
        }

        Ok(())
    }
}
