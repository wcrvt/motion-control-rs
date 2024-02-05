//https://users.rust-lang.org/t/how-to-save-a-vector-of-float-64/93728/4

use std::{error::Error, path::Path, fs::File};
use std::io::{BufWriter, Write};

pub struct DataStorage <T, P> {
  data: Vec<Vec<T>>,
  path: P,
  separator: String,
  cnt: usize
}

impl <T, P> DataStorage <T, P> where 
    T: std::fmt::Display + num_traits::Num + std::clone::Clone,
    P: AsRef<Path>
{
  pub fn new(path: P, separator: &str, row_size: usize, len: usize) -> Self {
    Self {
      data: vec![vec![T::zero(); row_size]; len],
      path,
      separator: separator.into(),
      cnt: 0
    }
  }

  pub fn add(&mut self, data: &[T]) {
    self.data[self.cnt] = Vec::from(data);
    self.cnt += 1;
  }

  pub fn write_file(&self) -> Result<(), Box<dyn Error>>{
    let file = File::create(&self.path.as_ref())?;
    let mut writer = BufWriter::new(file);

    for x in &self.data {
      let data_row = x
        .iter()
        .map(|x| x.to_string())
        .reduce(|x, y| x + &self.separator + &y)
        .unwrap();
      write!(writer, "{data_row}\n")?;
      writer.flush()?;
    }

    Ok(())
  }

}