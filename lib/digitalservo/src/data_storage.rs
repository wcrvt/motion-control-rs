use std::fmt::Debug;
use std::io::{BufWriter, Write};
use std::{error::Error, fs, fs::File, path::Path};

pub struct DataStorage<T, P, const ROWSIZE: usize> {
    data: Vec<Vec<T>>,
    path: P,
    separator: String,
    cnt: usize,
    datalen: usize,
}

impl<T, P, const ROWSIZE: usize> DataStorage<T, P, ROWSIZE>
where
    T: Default + Copy + Debug,
    P: AsRef<Path>,
{
    pub fn new(path: P, separator: &str, datalen: usize) -> Self {

        let mut parents: Vec<&Path> = vec![];
        let mut path_ref: &Path = path.as_ref();
        loop {
            match path_ref.parent() {
                Some(parent) => {
                    if parent.is_dir() || parent == Path::new("") {
                        break
                    };
                    parents.push(parent);
                    path_ref = parent;
                }
                None => {
                    break
                }
            }
        }

        parents.reverse();
        for p in parents {
            match fs::create_dir(p) {
                Ok(_) => {},
                Err(e) => panic!("{:?}: {}", p, e),
            }
        }

        Self {
            data: vec![vec![T::default(); ROWSIZE]; datalen],
            path,
            separator: separator.into(),
            cnt: 0,
            datalen
        }
    }

    pub fn add(&mut self, data: [T; ROWSIZE]) {
        if self.cnt < self.datalen {
            self.data[self.cnt] = data.to_vec();
            self.cnt += 1;
        }
    }

    pub fn write_file(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(&self.path.as_ref())?;
        let mut writer = BufWriter::new(file);

        for i in 0..self.cnt {
            let data_row = &self.data[i]
                .iter()
                .map(|x| format!("{:.06?}", x))
                .reduce(|x, y| x + &self.separator + &y)
                .unwrap();
            if i < self.cnt - 1 {
                write!(writer, "{data_row}\n")?;
            } else {
                write!(writer, "{data_row}")?;
            }
            writer.flush()?;
        }

        Ok(())
    }
}
